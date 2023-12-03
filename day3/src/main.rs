use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: PathBuf,
}

#[derive(Debug)]
struct Part {
    number: u32,
    row: usize,
    range: Range<usize>,
}

#[derive(Debug)]
struct Symbol {
    row: usize,
    column: usize,
}

fn find_parts(row: usize, column: usize, parts: &[Part]) -> Vec<&Part> {
    parts
        .iter()
        .filter(|&part| {
            part.row == row
                && (part.range.contains(&(column - 1))
                    || part.range.contains(&(column))
                    || part.range.contains(&(column + 1)))
        })
        .collect()
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);

    let mut parts = Vec::new();
    let mut symbols = Vec::new();

    for (row, text) in reader.lines().enumerate() {
        let text = text?;
        let mut partno = 0;
        let mut part_start = None;
        let len = text.len();
        for (column, c) in text.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                partno = partno * 10 + digit;
                if part_start.is_none() {
                    part_start = Some(column);
                }
            } else {
                if let Some(start) = part_start {
                    parts.push(Part {
                        number: partno,
                        row,
                        range: start..column,
                    });
                    partno = 0;
                    part_start = None;
                }
                if c != '.' {
                    symbols.push(Symbol { row, column });
                }
            }
        }
        if let Some(start) = part_start {
            parts.push(Part {
                number: partno,
                row,
                range: start..len,
            });
        }
    }

    let sum: u32 = symbols
        .into_iter()
        .map(|symbol| {
            let mut sum = 0;

            if symbol.row > 0 {
                sum += find_parts(symbol.row - 1, symbol.column, &parts)
                    .into_iter()
                    .map(|part| part.number)
                    .sum::<u32>();
            }
            sum += find_parts(symbol.row, symbol.column, &parts)
                .into_iter()
                .map(|part| part.number)
                .sum::<u32>();
            sum += find_parts(symbol.row + 1, symbol.column, &parts)
                .into_iter()
                .map(|part| part.number)
                .sum::<u32>();

            sum
        })
        .sum();

    println!("{sum}");

    Ok(())
}
