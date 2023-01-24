use std::time::Instant;

use ril::stores::SqliteStore;
use ril::{Status, Task, Type};

fn main() {
    const REPEAT: usize = 10;
    const MEASUREMENTS: usize = 15;

    // cleanup the database just to be sure
    let sqlite = SqliteStore::new();
    sqlite.delete().unwrap();

    let mut documents = [0; MEASUREMENTS];
    for i in 0..MEASUREMENTS {
        documents[i] = 2_usize.pow(i as u32);
    }
    // drop the mutability
    let documents = documents;

    // starts the benchmarks
    let mut measurements = Vec::new();
    for _ in 0..REPEAT {
        measurements.push(bench(documents));
    }
    println!("# nb_documents_indexed time_it_took standard_deviation");
    for i in 0..MEASUREMENTS {
        let mut total = 0;
        for r in 0..REPEAT {
            total += measurements[r][i];
        }
        let average = total as f64 / REPEAT as f64;
        let mut std_dev = 0.;
        for r in 0..REPEAT {
            std_dev += (average - measurements[r][i] as f64).powi(2);
        }
        std_dev = (std_dev / REPEAT as f64).sqrt();
        println!("  {:<20} {:<20} {:<20}", documents[i], average, std_dev);
    }
}

fn bench<const MEASUREMENTS: usize>(documents: [usize; MEASUREMENTS]) -> [u128; MEASUREMENTS] {
    let sqlite = SqliteStore::new();
    let mut tasks = tasks();

    let mut measurements = [Instant::now(); MEASUREMENTS];

    let start = Instant::now();
    let mut nb_documents = 0;
    for i in 0..MEASUREMENTS {
        let nb_doc_to_reach = documents[i];
        while nb_documents < nb_doc_to_reach {
            sqlite.insert(&tasks.next().unwrap()).unwrap();
            nb_documents += 1;
        }
        measurements[i] = Instant::now();
    }

    sqlite.delete().unwrap();

    let mut durations = [0_u128; MEASUREMENTS];
    for i in 0..MEASUREMENTS {
        durations[i] = (measurements[i] - start).as_nanos();
    }

    durations
}

fn tasks() -> impl Iterator<Item = Task> {
    let mut statuses = [
        Status::Enqueued,
        Status::Processing,
        Status::Succeeded,
        Status::Failed,
    ]
    .into_iter()
    .cycle();

    let mut types = [
        Type::IndexCreation,
        Type::IndexDeletion,
        Type::IndexSwap,
        Type::DocumentAddition,
        Type::DocumentDeletion,
    ]
    .into_iter()
    .cycle();

    (0..).map(move |id| Task {
        id,
        status: statuses.next().unwrap(),
        r#type: types.next().unwrap(),
    })
}
