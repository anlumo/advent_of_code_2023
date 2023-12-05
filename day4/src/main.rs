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

#[derive(Default, Debug)]
struct Scratchcard {
    win_count: usize,
    card_count: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);

    let mut cards: Vec<_> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let Some((_, numbers)) = line.split_once(": ") else {
                return Scratchcard::default();
            };
            let Some((winning, have)) = numbers.split_once(" | ") else {
                return Scratchcard::default();
            };
            let winning: HashSet<_> = winning
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            let win_count = have
                .split_whitespace()
                .filter(|&n| winning.contains(&n.parse::<usize>().unwrap()))
                .count();
            Scratchcard {
                win_count,
                card_count: 1,
            }
        })
        .collect();

    let sum = (0..cards.len())
        .map(|idx| {
            let Scratchcard {
                win_count,
                card_count,
            } = cards[idx];
            for won in cards[(idx + 1)..(idx + 1 + win_count)].iter_mut() {
                won.card_count += card_count;
            }
            card_count
        })
        .sum::<usize>();

    println!("{sum}");

    Ok(())
}
