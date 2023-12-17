#![allow(unused)]

use std::{
    fmt::Debug,
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

#[derive(Clone, Copy, PartialEq, Eq)]
struct Row {
    bits: u64,
    width: usize,
}

impl Row {
    fn new(input: impl IntoIterator<Item = bool>) -> Self {
        let mut num = 0;
        let mut idx = 0;
        for i in input {
            if i {
                num |= 1 << idx;
            }
            idx += 1;
        }
        Self {
            bits: num,
            width: idx,
        }
    }
    fn at(&self, index: usize) -> bool {
        self.bits & (1 << index) != 0
    }
}

impl std::fmt::Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.width)
                .map(|idx| if self.bits & (1 << idx) != 0 {
                    '\u{2588}'
                } else {
                    '\u{2591}'
                })
                .collect::<String>(),
        )
    }
}

// only works with 0-based rows!
fn transpose(pattern: &[Row]) -> Vec<Row> {
    let mut result = vec![
        Row {
            bits: 0,
            width: pattern.len(),
        };
        pattern[0].width
    ];

    for (idx, row) in pattern.iter().enumerate() {
        for (new_row, col) in result.iter_mut().enumerate() {
            if row.bits & (1 << new_row) != 0 {
                col.bits |= 1 << idx;
            }
        }
    }

    result
}

fn check_symmetry(pattern: &[Row]) -> Vec<usize> {
    // println!("check_symmetry {pattern:#?}");
    (1..pattern.len())
        .filter(|&y| {
            (y..pattern.len()).all(|y2| {
                // println!("y = {y}, y2 = {y2}");
                if 2 * y < y2 + 1 {
                    true
                } else {
                    pattern[y2] == pattern[2 * y - y2 - 1]
                }
            })
            // println!("result = {result:?}");
        })
        .collect()
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);

    let mut patterns = Vec::new();
    let mut current_pattern = Vec::<Row>::new();
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            patterns.push(current_pattern);
            current_pattern = Vec::new();
            continue;
        }
        current_pattern.push(Row::new(line.chars().map(|c| c == '#')));
    }
    if !current_pattern.is_empty() {
        patterns.push(current_pattern);
    }

    let result = patterns
        .iter()
        .map(|pattern| {
            // eprintln!("{pattern:#?}");
            let sym_v = check_symmetry(pattern);
            let transposed = transpose(pattern);
            let sym_h = check_symmetry(&transposed);

            sym_v.iter().sum::<usize>() * 100 + sym_h.iter().sum::<usize>()
        })
        .sum::<usize>();

    println!("{result:?}");

    Ok(())
}
