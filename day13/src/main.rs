use std::{
    collections::HashSet,
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

fn check_symmetry(pattern: &[Row]) -> HashSet<usize> {
    (1..pattern.len())
        .filter(|&y| {
            (y..pattern.len()).all(|y2| {
                if 2 * y < y2 + 1 {
                    true
                } else {
                    pattern[y2] == pattern[2 * y - y2 - 1]
                }
            })
        })
        .collect()
}

fn fix_smudge(pattern: &[Row], location: usize) -> Vec<Row> {
    let x = location % pattern[0].width;
    let y = location / pattern[0].width;
    pattern
        .iter()
        .enumerate()
        .map(|(idx, row)| {
            if idx == y {
                Row {
                    bits: row.bits ^ (1 << x),
                    width: row.width,
                }
            } else {
                *row
            }
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
            let original_sym_v = check_symmetry(pattern);
            let transposed = transpose(pattern);
            let original_sym_h = check_symmetry(&transposed);

            let mut sym_v = HashSet::new();
            let mut sym_h = HashSet::new();

            for location in 0..(pattern[0].width * pattern.len()) {
                let fixed = fix_smudge(pattern, location);
                let new_sym_v = check_symmetry(&fixed);
                let transposed = transpose(&fixed);
                let new_sym_h = check_symmetry(&transposed);

                sym_v.extend(new_sym_v);
                sym_h.extend(new_sym_h);
            }
            let sym_v: Vec<_> = sym_v.difference(&original_sym_v).collect();
            let sym_h: Vec<_> = sym_h.difference(&original_sym_h).collect();
            sym_v.iter().copied().sum::<usize>() * 100 + sym_h.iter().copied().sum::<usize>()
        })
        .sum::<usize>();

    println!("{result:?}");

    Ok(())
}
