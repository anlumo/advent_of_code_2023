use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: PathBuf,
}

fn derive(sequence: &[isize]) -> Vec<isize> {
    sequence.windows(2).map(|a| a[1] - a[0]).collect()
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);

    let input: Vec<_> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|num| num.parse::<isize>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        })
        .collect();

    let result = input
        .into_iter()
        .map(|sequence| {
            let mut steps = vec![sequence];
            while steps
                .last()
                .map(|step| step.iter().any(|&num| num != 0))
                .unwrap()
            {
                let prev = steps.last().unwrap();
                steps.push(derive(prev));
            }

            if let Some(last_step) = steps.last_mut() {
                last_step.push(0);
            }

            steps.reverse();

            for idx in 0..(steps.len() - 1) {
                let prev = *steps[idx].first().unwrap();
                let cur = &mut steps[idx + 1];
                let value = *cur.first().unwrap();
                cur.insert(0, value - prev);
            }

            *steps.last().unwrap().first().unwrap()
        })
        .sum::<isize>();

    println!("{result}");

    Ok(())
}
