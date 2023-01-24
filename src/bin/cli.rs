use std::time::Instant;

use clap::Parser;
use ril::options::Option;
use ril::stores::{CustomStore, SqliteStore, Store};

fn main() {
    let opt = Option::parse();

    let custom = CustomStore::new();
    let sqlite = SqliteStore::new();

    match opt {
        Option::Insert(task) => {
            let now = Instant::now();
            custom.insert(&task).unwrap();
            let custom = now.elapsed();
            let now = Instant::now();
            sqlite.insert(&task).unwrap();
            let sqlite = now.elapsed();

            println!("Inserting in the custom db took {custom:?}");
            println!("Inserting in the sqlite db took {sqlite:?}");
        }
        Option::Query(query) => {
            let now = Instant::now();
            dbg!(custom.query(&query).unwrap());
            let custom = now.elapsed();
            let now = Instant::now();
            dbg!(sqlite.query(&query).unwrap());
            let sqlite = now.elapsed();

            println!("Querying in the custom db took {custom:?}");
            println!("Querying in the sqlite db took {sqlite:?}");
        }
    }
}
