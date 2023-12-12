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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Clone)]
struct Row {
    group_info: Vec<Spring>,
    configuration: Vec<usize>,
}

impl std::fmt::Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Row {}]",
            self.group_info
                .iter()
                .map(|&s| match s {
                    Spring::Damaged => '#',
                    Spring::Operational => '.',
                    Spring::Unknown => '?',
                })
                .collect::<String>()
        )
    }
}

fn validate_row(row: &Row) -> bool {
    let mut group_iter = row.group_info.iter();
    let mut row_iter = row.configuration.iter().fuse();
    let Some(mut group_next) = group_iter.next() else {
        return false;
    };
    let Some(group_size) = row_iter.next() else {
        return false;
    };
    let mut group_size = *group_size;
    loop {
        match group_next {
            Spring::Operational => {
                if let Some(next) = group_iter.next() {
                    group_next = next;
                } else {
                    return false;
                }
            }
            Spring::Damaged => {
                for _ in 1..group_size {
                    if !matches!(group_iter.next(), Some(&Spring::Damaged)) {
                        return false;
                    }
                }
                let Some(&damaged) = row_iter.next() else {
                    // end of the configurations, flush rest of the row
                    for group_next in group_iter {
                        if *group_next != Spring::Operational {
                            return false;
                        }
                    }
                    return true;
                };
                group_size = damaged;
                // next entry in row must be operational
                let Some(next) = group_iter.next() else {
                    return false;
                };
                if *next != Spring::Operational {
                    return false;
                }
                group_next = next;
            }
            Spring::Unknown => {
                return false;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);

    let rows: Vec<Row> = reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let (groups, config) = line.split_once(' ')?;
            Some(Row {
                group_info: groups
                    .chars()
                    .map(|c| match c {
                        '.' => Spring::Operational,
                        '#' => Spring::Damaged,
                        '?' => Spring::Unknown,
                        _ => panic!("Unknown input"),
                    })
                    .collect(),
                configuration: config
                    .split(',')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect(),
            })
        })
        .collect();

    let result = rows
        .iter()
        .map(|row| {
            let unknowns = row
                .group_info
                .iter()
                .filter(|&spring| *spring == Spring::Unknown)
                .count();
            (0..2usize.pow(unknowns as _))
                .filter(|bits| {
                    let mut bit_idx = 0;
                    let test_row = Row {
                        group_info: row
                            .group_info
                            .iter()
                            .map(|&spring| {
                                if spring != Spring::Unknown {
                                    spring
                                } else if bits & (1 << bit_idx) != 0 {
                                    bit_idx += 1;
                                    Spring::Damaged
                                } else {
                                    bit_idx += 1;
                                    Spring::Operational
                                }
                            })
                            .collect(),
                        configuration: row.configuration.clone(),
                    };
                    validate_row(&test_row)
                })
                .count()
        })
        .sum::<usize>();

    println!("{result}");
    Ok(())
}
