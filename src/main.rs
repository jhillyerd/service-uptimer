use std::env;
use std::fs::File;
use std::io::{BufReader};

mod config;

fn main() {
    // Parse args.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <config file>", args[0]);
        std::process::exit(1);
    }
    let file_name = &args[1];

    // Load config.
    let file = File::open(file_name).expect("failed to open config");
    let reader = BufReader::new(file);
    let realm = config::Realm::from_reader(reader);
    println!("{:?}", realm);
}
