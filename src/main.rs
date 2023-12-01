use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Parser)]
struct Args {
    filename: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);

    let sum: usize = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let first = line.chars().find(|c| c.is_ascii_digit());
            let last = line.chars().rfind(|c| c.is_ascii_digit());
            if let (Some(first), Some(last)) = (first, last) {
                format!("{first}{last}").parse().unwrap()
            } else {
                eprintln!("Line with no number!");
                0
            }
        })
        .sum();

    println!("{sum}");

    Ok(())
}
