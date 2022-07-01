use std::io;
use std::io::Write;

use clap::Parser;
use csv;

/// Simple program to quiz user, questions and answers provided via a CSV file
#[derive(Parser, Default, Debug)]
#[clap(about)]
struct Args {
    /// Number of times to greet
    #[clap(short, long, value_parser, default_value = "problems.csv")]
    filename: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}

type Record = (String, String);

fn main() {
    let args = Args::parse();

    // Creates a new csv `Reader` from a file
    let mut reader = csv::Reader::from_path(args.filename).expect("CSV file");

    let mut problems: Vec<Record> = Vec::new();

    // `.deserialize` returns an iterator of the internal
    // record structure deserialized
    for result in reader.deserialize() {
        let record: Record = result.expect("a CSV record");
        problems.push(record)
    }

    let mut correct = 0;
    for (i, problem) in problems.iter().enumerate() {
        println!("Problem {}: {}", i+1, problem.0);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let answer = input.trim();
        if problem.1.trim() == answer {
            correct += 1;
        }
    }

    println!("You have scored {} out of {}.", correct, problems.len())
}
