use std::io::{Read, BufRead, BufReader};
use std::fs::File;
use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let reader = BufReader::new(file);

    find_matches(reader, &args.pattern, &mut std::io::stdout());

    Ok(())
}

fn find_matches<R: Read>(reader: BufReader<R>, pattern: &str, mut writer: impl std::io::Write) {
    for wrapped_line in reader.lines() {
        let line = wrapped_line.unwrap();
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let reader = BufReader::new("lorem ipsum\ndolor sit amet".as_bytes());
    find_matches(reader, "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}