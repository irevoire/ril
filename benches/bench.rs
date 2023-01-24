use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use indicatif::ProgressIterator;
use ril::stores::{CustomStore, SqliteStore, Store};
use ril::{Status, Task, Type};

fn main() {
    const REPEAT: usize = 10;
    const MEASUREMENTS: usize = 15;

    // we're going to open the end file at the very beginning just to trigger the possible error before running the benchmark.
    let file = File::create("bench.dat").unwrap();

    let mut documents = [0; MEASUREMENTS];
    for i in 0..MEASUREMENTS {
        documents[i] = 2_usize.pow(i as u32);
    }
    // drop the mutability
    let documents = documents;

    // ====== SQLITE

    println!("Benchmarking sqlite...");
    // cleanup the database just to be sure
    let sqlite = SqliteStore::new();
    sqlite.delete().unwrap();

    // starts the benchmarks
    let mut sqlite_measurements = Vec::new();
    for _ in (0..REPEAT).progress() {
        sqlite_measurements.push(bench(SqliteStore::new(), documents));
    }

    println!("Benchmarking our custom store...");
    // cleanup the database just to be sure
    let custom = CustomStore::new();
    custom.delete().unwrap();

    // starts the benchmarks
    let mut custom_measurements = Vec::new();
    for _ in (0..REPEAT).progress() {
        custom_measurements.push(bench(CustomStore::new(), documents));
    }

    // Writing the results on disk
    let mut writer = BufWriter::new(file);
    writeln!(writer, "# nb_documents_indexed sqlite_time_it_took sqlite_standard_deviation custom_time_it_took custom_standard_deviation").unwrap();
    for i in 0..MEASUREMENTS {
        let mut sqlite_total = 0;
        let mut custom_total = 0;
        for r in 0..REPEAT {
            sqlite_total += sqlite_measurements[r][i];
            custom_total += custom_measurements[r][i];
        }
        let sqlite_average = sqlite_total as f64 / REPEAT as f64;
        let custom_average = custom_total as f64 / REPEAT as f64;
        let mut sqlite_std_dev = 0.;
        let mut custom_std_dev = 0.;
        for r in 0..REPEAT {
            sqlite_std_dev += (sqlite_average - sqlite_measurements[r][i] as f64).powi(2);
            custom_std_dev += (custom_average - custom_measurements[r][i] as f64).powi(2);
        }
        sqlite_std_dev = (sqlite_std_dev / REPEAT as f64).sqrt();
        custom_std_dev = (custom_std_dev / REPEAT as f64).sqrt();
        writeln!(
            writer,
            "  {:<20} {:<20} {:<20} {:<20} {:<20}",
            documents[i], sqlite_average, sqlite_std_dev, custom_average, custom_std_dev
        )
        .unwrap();
    }
    writer.flush().unwrap();

    println!("Result has been written on disk in `bench.dat`.");
    println!("Run `gnuplot -c plot.gnuplot` to visualize the data.");
}

fn bench<const MEASUREMENTS: usize>(
    store: impl Store,
    documents: [usize; MEASUREMENTS],
) -> [u128; MEASUREMENTS] {
    let mut tasks = tasks();

    let mut measurements = [Instant::now(); MEASUREMENTS];

    let mut nb_documents = 0;
    let start = Instant::now();
    // in this loop we wants to reduce to the maximum the number of operation unrelated to the
    // store we're doing.
    for i in 0..MEASUREMENTS {
        let nb_doc_to_reach = documents[i];
        while nb_documents < nb_doc_to_reach {
            store.insert(&tasks.next().unwrap()).unwrap();
            nb_documents += 1;
        }
        measurements[i] = Instant::now();
    }

    store.delete().unwrap();

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
