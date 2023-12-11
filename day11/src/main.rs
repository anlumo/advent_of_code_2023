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

const EXPANSION: isize = 999_999; // one less, because we have to *replace* the existing empty row/col

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);

    let universe = Universe {
        field: reader
            .lines()
            .filter_map(|line| Some(line.ok()?.chars().map(|c| c == '#').collect::<Vec<_>>()))
            .collect(),
    };

    // expand rows
    let empty_rows: Vec<_> = universe
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

    // expand columns
    let empty_cols: Vec<_> = (0..universe.width())
        .filter(|&x| (0..universe.height()).all(|y| !universe[(x, y)]))
        .collect();

    let galaxies: Vec<_> = universe
        .field
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, &c)| {
                    if c {
                        let expansion_x =
                            empty_cols.iter().filter(|&idx| *idx < x).count() as isize * EXPANSION;
                        let expansion_y =
                            empty_rows.iter().filter(|&idx| *idx < y).count() as isize * EXPANSION;
                        Some((x as isize + expansion_x, y as isize + expansion_y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let result = galaxies
        .iter()
        .enumerate()
        .flat_map(|(idx, galaxy1)| {
            galaxies[idx + 1..].iter().map(move |galaxy2| {
                // Manhattan distance
                (galaxy1.0 - galaxy2.0).abs() + (galaxy1.1 - galaxy2.1).abs()
            })
        })
        .sum::<isize>();

    println!("{result:?}");

    Ok(())
}
