use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);

    let sum = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let Some((_, numbers)) = line.split_once(": ") else {
                return 0;
            };
            let Some((winning, have)) = numbers.split_once(" | ") else {
                return 0;
            };
            let winning: HashSet<_> = winning
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            let winners_count = have
                .split_whitespace()
                .filter(|&n| winning.contains(&n.parse::<usize>().unwrap()))
                .count();
            if winners_count == 0 {
                0
            } else {
                2usize.pow((winners_count - 1) as u32)
            }
        })
        .sum::<usize>();

    println!("{sum}");

    Ok(())
}
