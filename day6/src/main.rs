use std::{
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
    let mut lines = reader.lines();

    let times: Vec<_> = lines
        .next()
        .expect("time line missing")
        .expect("line 1 is not text")
        .split_whitespace()
        .filter_map(|t| t.parse::<usize>().ok())
        .collect();

    let distance: Vec<_> = lines
        .next()
        .expect("distance line missing")
        .expect("line 2 is not text")
        .split_whitespace()
        .filter_map(|t| t.parse::<usize>().ok())
        .collect();

    let result = times
        .into_iter()
        .zip(distance)
        .map(|(time, distance)| {
            (0..time)
                .map(|acceleration| (acceleration + 1) * (time - acceleration - 1))
                .filter(|&my_distance| my_distance > distance)
                .count()
        })
        .reduce(|prev, item| prev * item)
        .unwrap();

    println!("{result}");

    Ok(())
}
