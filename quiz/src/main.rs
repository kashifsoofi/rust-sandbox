use std::io;
use std::io::Write;

use clap::Parser;
use csv;

use tokio::{self, time};
use tokio::sync::mpsc;
use tokio::time::Duration;

/// Simple program to quiz user, questions and answers provided via a CSV file
#[derive(Parser, Default, Debug)]
#[clap(about)]
struct Args {
    /// Number of times to greet
    #[clap(short, long, value_parser, default_value = "problems.csv")]
    filename: String,

    /// the time limit for the quiz in seconds
    #[clap(short, long, value_parser, default_value_t = 30)]
    time_limit: u64,
}

type Record = (String, String);

async fn wait(time_out: u64, sender: mpsc::Sender<()>) {
    time::sleep(Duration::from_millis(time_out * 1000)).await;
    sender.send(()).await.expect("send timeout");
}

async fn quiz(problems: Vec<Record>, sender: mpsc::Sender<i32>) {
    for (i, problem) in problems.iter().enumerate() {
        println!("Problem {}: {}", i+1, problem.0);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let answer = input.trim();
        if problem.1.trim() == answer {
            sender.send(1).await.expect("answer matched");
        } else {
            sender.send(0).await.expect("answer not matched");
        }
    }
}

#[tokio::main]
async fn main() {
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

    let total_problems = problems.len();

    let (tx, mut rx) = mpsc::channel::<i32>(1);
    let (tx2, mut rx2) = mpsc::channel(1);

    let t = tokio::spawn(quiz(problems, tx));
    tokio::spawn(wait(args.time_limit, tx2));

    let mut correct = 0;
    let mut count = 0;
    loop {
        tokio::select! {
            r = rx.recv() => {
                correct += r.unwrap();
                count += 1;
                if count == total_problems {
                    break;
                }
            }
            _ = rx2.recv() => {
                t.abort();
                break;
            }
        }
    }
    
    println!("You have scored {} out of {}.", correct, total_problems);
    std::process::exit(1)
}
