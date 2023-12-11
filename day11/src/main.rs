use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    ops::Index,
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: PathBuf,
}

struct Universe {
    field: Vec<Vec<bool>>,
}

impl Index<(usize, usize)> for Universe {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.field[index.1][index.0]
    }
}

impl Universe {
    fn height(&self) -> usize {
        self.field.len()
    }
    fn width(&self) -> usize {
        self.field[0].len()
    }
}

impl std::fmt::Debug for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.field {
            writeln!(
                f,
                "{}",
                row.iter()
                    .map(|&x| if x { '#' } else { '.' })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);

    let mut universe = Universe {
        field: reader
            .lines()
            .filter_map(|line| Some(line.ok()?.chars().map(|c| c == '#').collect::<Vec<_>>()))
            .collect(),
    };

    // expand rows
    let mut empty_rows: Vec<_> = universe
        .field
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            if row.iter().all(|&c| !c) {
                Some(idx)
            } else {
                None
            }
        })
        .collect();
    let width = universe.width();
    empty_rows.reverse();
    for idx in empty_rows {
        universe.field.insert(idx, vec![false; width]);
    }

    // expand columns

    let mut empty_cols: Vec<_> = (0..universe.width())
        .filter(|&x| (0..universe.height()).all(|y| !universe[(x, y)]))
        .collect();

    empty_cols.reverse();

    for x in empty_cols {
        for row in &mut universe.field {
            row.insert(x, false);
        }
    }

    let galaxies: Vec<_> = universe
        .field
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, &c)| {
                    if c {
                        Some((x as isize, y as isize))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let result = galaxies
        .iter()
        .flat_map(|galaxy1| {
            galaxies.iter().map(|galaxy2| {
                // Manhattan distance
                (galaxy1.0 - galaxy2.0).abs() + (galaxy1.1 - galaxy2.1).abs()
            })
        })
        .sum::<isize>()
        / 2;

    println!("{result:?}");

    Ok(())
}
