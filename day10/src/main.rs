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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn follow(
    field: &[Vec<Tile>],
    prev_pos: (usize, usize),
    direction: Direction,
) -> Option<((usize, usize), Direction)> {
    let pos = match direction {
        Direction::North => (prev_pos.0, prev_pos.1 + 1),
        Direction::East => (prev_pos.0 - 1, prev_pos.1),
        Direction::South => (prev_pos.0, prev_pos.1 - 1),
        Direction::West => (prev_pos.0 + 1, prev_pos.1),
    };
    // eprintln!(
    //     "{prev_pos:?} => Position {pos:?} tile {:?} coming from {direction:?}",
    //     field[pos.1][pos.0]
    // );
    match field[pos.1][pos.0] {
        Tile::VerticalPipe => match direction {
            Direction::North => Some((pos, Direction::North)),
            Direction::East => unreachable!(),
            Direction::South => Some((pos, Direction::South)),
            Direction::West => unreachable!(),
        },
        Tile::HorizontalPipe => match direction {
            Direction::North => unreachable!(),
            Direction::East => Some((pos, Direction::East)),
            Direction::South => unreachable!(),
            Direction::West => Some((pos, Direction::West)),
        },
        Tile::NorthEastBend => match direction {
            Direction::North => Some((pos, Direction::West)),
            Direction::East => Some((pos, Direction::South)),
            Direction::South => unreachable!(),
            Direction::West => unreachable!(),
        },
        Tile::NorthWestBend => match direction {
            Direction::North => Some((pos, Direction::East)),
            Direction::East => unreachable!(),
            Direction::South => unreachable!(),
            Direction::West => Some((pos, Direction::South)),
        },
        Tile::SouthWestBend => match direction {
            Direction::North => unreachable!(),
            Direction::East => unreachable!(),
            Direction::South => Some((pos, Direction::East)),
            Direction::West => Some((pos, Direction::North)),
        },
        Tile::SouthEastBend => match direction {
            Direction::North => unreachable!(),
            Direction::East => Some((pos, Direction::North)),
            Direction::South => Some((pos, Direction::West)),
            Direction::West => unreachable!(),
        },
        Tile::Ground => panic!("Ran into the ground at {pos:?}!"),
        Tile::Start => None,
    }
}

fn find_start(field: &[Vec<Tile>]) -> Option<((usize, usize), Direction)> {
    for (y, row) in field.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == Tile::Start {
                // scan around the start for connecting pipes
                if x > 0 {
                    let left = field[y][x - 1];
                    let start = (x - 1, y);
                    match left {
                        Tile::HorizontalPipe => return follow(field, start, Direction::West),
                        Tile::NorthEastBend => return follow(field, start, Direction::South),
                        Tile::SouthEastBend => return follow(field, start, Direction::North),
                        _ => {}
                    }
                }
                if y > 0 {
                    let top = field[y - 1][x];
                    let start = (x, y - 1);
                    match top {
                        Tile::VerticalPipe => return follow(field, start, Direction::North),
                        Tile::SouthWestBend => return follow(field, start, Direction::East),
                        Tile::SouthEastBend => return follow(field, start, Direction::West),
                        _ => {}
                    }
                }
                if x < field.len() - 1 {
                    let right = field[y][x + 1];
                    let start = (x + 1, y);
                    match right {
                        Tile::HorizontalPipe => return follow(field, start, Direction::East),
                        Tile::NorthWestBend => return follow(field, start, Direction::South),
                        Tile::SouthWestBend => return follow(field, start, Direction::North),
                        _ => {}
                    }
                }
                if y < field[0].len() - 1 {
                    let bottom = field[y + 1][x];
                    let start = (x, y + 1);
                    match bottom {
                        Tile::VerticalPipe => return follow(field, start, Direction::South),
                        Tile::NorthWestBend => return follow(field, start, Direction::East),
                        Tile::NorthEastBend => return follow(field, start, Direction::West),
                        _ => {}
                    }
                }

                eprintln!("Start field has no connecting tiles");

                return None;
            }
        }
    }
    None
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);

    let field: Vec<_> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    '|' => Tile::VerticalPipe,
                    '-' => Tile::HorizontalPipe,
                    'L' => Tile::NorthEastBend,
                    'J' => Tile::NorthWestBend,
                    '7' => Tile::SouthWestBend,
                    'F' => Tile::SouthEastBend,
                    '.' => Tile::Ground,
                    'S' => Tile::Start,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    if let Some((mut pos, mut direction)) = find_start(&field) {
        let mut walk = vec![(pos, direction)];
        while let Some(step) = follow(&field, pos, direction) {
            pos = step.0;
            direction = step.1;
            walk.push(step);
        }
        // println!("{walk:?}");
        println!("{}", 1 + (walk.len() + 1) / 2);
        return Ok(());
    }

    eprintln!("Start not found");

    Ok(())
}
