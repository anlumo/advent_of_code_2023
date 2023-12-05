use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
    path::PathBuf,
    time::Instant,
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: PathBuf,
}

struct Mapper {
    source: Range<i64>,
    offset: i64,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let now = Instant::now();

    let Some(Ok(seeds)) = lines.next() else {
        eprintln!("No seeds");
        return Ok(());
    };
    let seeds: Vec<_> = seeds
        .split_whitespace()
        .filter_map(|seed| seed.parse::<i64>().ok())
        .collect();

    let seeds: Vec<_> = seeds
        .chunks_exact(2)
        .map(|seed| seed[0]..(seed[1] + seed[0]))
        .collect();

    let mut mappings = Vec::<Vec<Mapper>>::new();

    let mut current_mapping = Vec::new();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            if !current_mapping.is_empty() {
                mappings.push(current_mapping);
                current_mapping = Vec::new();
            }
        } else if let Some(c) = line.chars().next() {
            if c.is_ascii_digit() {
                let numbers: Vec<_> = line
                    .split_whitespace()
                    .filter_map(|num| num.parse::<i64>().ok())
                    .collect();
                if numbers.len() != 3 {
                    eprintln!("Line with non-3 numbers encountered: {line}");
                    return Ok(());
                }
                current_mapping.push(Mapper {
                    source: numbers[1]..(numbers[1] + numbers[2]),
                    offset: numbers[0] - numbers[1],
                });
            }
        }
    }
    if !current_mapping.is_empty() {
        mappings.push(current_mapping);
    }

    let locations = seeds.into_iter().flatten().map(|mut seed| {
        for mapping in mappings.iter() {
            for map in mapping {
                if map.source.contains(&seed) {
                    seed += map.offset;
                    break;
                }
            }
        }
        seed
    });

    println!("lowest: {}", locations.min().unwrap());

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
