use std::io;
use std::io::Write;

use clap::Parser;
use csv;

use tokio::{self, task, time};
use tokio::sync::{oneshot, mpsc};
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

async fn wait(time_out: u64, sender: oneshot::Sender<bool>) {
    time::sleep(Duration::from_millis(time_out * 1000)).await;
    sender.send(true);
}

async fn get_answer(sender: oneshot::Sender<String>) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let answer = input.trim();
    sender.send(answer.to_string());
}

async fn quiz(problems: Vec<Record>, sender: mpsc::Sender<bool>) {
    for (i, problem) in problems.iter().enumerate() {
        println!("Problem {}: {}", i+1, problem.0);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let answer = input.trim();
        if problem.1.trim() == answer {
            sender.send(true);
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

    let (tx, rx) = mpsc::channel::<bool>(1);
    let (tx2, rx2) = oneshot::channel::<bool>();

    task::spawn(quiz(problems.clone(), tx));
    task::spawn(wait(args.time_limit, tx2));

    let mut correct = 0;
    for (i, problem) in problems.iter().enumerate() {
        println!("Problem {}: {}", i+1, problem.0);
        io::stdout().flush().unwrap();

        let (tx3, rx3) = oneshot::channel::<String>();
        task::spawn(get_answer(tx3));

        tokio::select! {
            answer = rx3 => {
                let unwraped_answer = answer.unwrap();
                if unwraped_answer == problem.1 {
                    correct += 1;
                }
            }
            _ = rx2 => {
                break;
            }
        }
    }
    
    // for (i, problem) in problems.iter().enumerate() {
    //     println!("Problem {}: {}", i+1, problem.0);
    //     io::stdout().flush().unwrap();

    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).expect("Failed to read line");

    //     let answer = input.trim();
    //     if problem.1.trim() == answer {
    //         correct += 1;
    //     }
    // }

    println!("You have scored {} out of {}.", correct, problems.len())
}
