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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Move {
    pos: (usize, usize),
    direction: Direction,
}

fn follow(field: &[Vec<Tile>], prev_move: Move) -> Option<Move> {
    let pos = match prev_move.direction {
        Direction::North => (prev_move.pos.0, prev_move.pos.1 + 1),
        Direction::East => (prev_move.pos.0 - 1, prev_move.pos.1),
        Direction::South => (prev_move.pos.0, prev_move.pos.1 - 1),
        Direction::West => (prev_move.pos.0 + 1, prev_move.pos.1),
    };
    match field[pos.1][pos.0] {
        Tile::VerticalPipe => match prev_move.direction {
            Direction::North => Some(Move {
                pos,
                direction: Direction::North,
            }),
            Direction::East => unreachable!(),
            Direction::South => Some(Move {
                pos,
                direction: Direction::South,
            }),
            Direction::West => unreachable!(),
        },
        Tile::HorizontalPipe => match prev_move.direction {
            Direction::North => unreachable!(),
            Direction::East => Some(Move {
                pos,
                direction: Direction::East,
            }),
            Direction::South => unreachable!(),
            Direction::West => Some(Move {
                pos,
                direction: Direction::West,
            }),
        },
        Tile::NorthEastBend => match prev_move.direction {
            Direction::North => Some(Move {
                pos,
                direction: Direction::West,
            }),
            Direction::East => Some(Move {
                pos,
                direction: Direction::South,
            }),
            Direction::South => unreachable!(),
            Direction::West => unreachable!(),
        },
        Tile::NorthWestBend => match prev_move.direction {
            Direction::North => Some(Move {
                pos,
                direction: Direction::East,
            }),
            Direction::East => unreachable!(),
            Direction::South => unreachable!(),
            Direction::West => Some(Move {
                pos,
                direction: Direction::South,
            }),
        },
        Tile::SouthWestBend => match prev_move.direction {
            Direction::North => unreachable!(),
            Direction::East => unreachable!(),
            Direction::South => Some(Move {
                pos,
                direction: Direction::East,
            }),
            Direction::West => Some(Move {
                pos,
                direction: Direction::North,
            }),
        },
        Tile::SouthEastBend => match prev_move.direction {
            Direction::North => unreachable!(),
            Direction::East => Some(Move {
                pos,
                direction: Direction::North,
            }),
            Direction::South => Some(Move {
                pos,
                direction: Direction::West,
            }),
            Direction::West => unreachable!(),
        },
        Tile::Ground => panic!("Ran into the ground at {pos:?}!"),
        Tile::Start => None,
    }
}

fn find_start(field: &[Vec<Tile>]) -> Option<[Move; 2]> {
    for (y, row) in field.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == Tile::Start {
                let start = (x, y);
                // scan around the start for connecting pipes
                if x > 0 {
                    let left = field[y][x - 1];
                    let pos = (x - 1, y);
                    match left {
                        Tile::HorizontalPipe => {
                            return Some([
                                Move {
                                    pos: start,
                                    direction: Direction::East,
                                },
                                Move {
                                    pos,
                                    direction: Direction::East,
                                },
                            ]);
                        }
                        Tile::NorthEastBend => {
                            return Some([
                                Move {
                                    pos: start,
                                    direction: Direction::West,
                                },
                                Move {
                                    pos,
                                    direction: Direction::South,
                                },
                            ]);
                        }
                        Tile::SouthEastBend => {
                            return Some([
                                Move {
                                    pos: start,
                                    direction: Direction::West,
                                },
                                Move {
                                    pos,
                                    direction: Direction::North,
                                },
                            ]);
                        }
                        _ => {}
                    }
                }
                if y > 0 {
                    let top = field[y - 1][x];
                    let pos = (x, y - 1);
                    match top {
                        Tile::VerticalPipe => {
                            return Some([
                                Move {
                                    pos: start,
                                    direction: Direction::South,
                                },
                                Move {
                                    pos,
                                    direction: Direction::South,
                                },
                            ]);
                        }
                        Tile::SouthWestBend => {
                            return Some([
                                Move {
                                    pos: start,
                                    direction: Direction::North,
                                },
                                Move {
                                    pos,
                                    direction: Direction::East,
                                },
                            ]);
                        }
                        Tile::SouthEastBend => {
                            return Some([
                                Move {
                                    pos: start,
                                    direction: Direction::North,
                                },
                                Move {
                                    pos,
                                    direction: Direction::West,
                                },
                            ]);
                        }
                        _ => {}
                    }
                }
                if x < field.len() - 1 {
                    let right = field[y][x + 1];
                    let pos = (x + 1, y);
                    match right {
                        Tile::HorizontalPipe => {
                            return Some([
                                Move {
                                    pos: start,
                                    direction: Direction::West,
                                },
                                Move {
                                    pos,
                                    direction: Direction::West,
                                },
                            ]);
                        }
                        Tile::NorthWestBend => {
                            return Some([
                                Move {
                                    pos: start,
                                    direction: Direction::East,
                                },
                                Move {
                                    pos,
                                    direction: Direction::South,
                                },
                            ]);
                        }
                        Tile::SouthWestBend => {
                            return Some([
                                Move {
                                    pos: start,
                                    direction: Direction::East,
                                },
                                Move {
                                    pos,
                                    direction: Direction::North,
                                },
                            ]);
                        }
                        _ => {}
                    }
                }
                if y < field[0].len() - 1 {
                    let bottom = field[y + 1][x];
                    let pos = (x, y + 1);
                    match bottom {
                        Tile::VerticalPipe => {
                            return Some([
                                Move {
                                    pos: start,
                                    direction: Direction::North,
                                },
                                Move {
                                    pos,
                                    direction: Direction::North,
                                },
                            ]);
                        }
                        Tile::NorthWestBend => {
                            return Some([
                                Move {
                                    pos: start,
                                    direction: Direction::East,
                                },
                                Move {
                                    pos,
                                    direction: Direction::East,
                                },
                            ]);
                        }
                        Tile::NorthEastBend => {
                            return Some([
                                Move {
                                    pos: start,
                                    direction: Direction::South,
                                },
                                Move {
                                    pos,
                                    direction: Direction::West,
                                },
                            ]);
                        }
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

    let Some(start) = find_start(&field) else {
        eprintln!("Start not found");
        return Ok(());
    };
    let mut walk = start.to_vec();
    let mut pos = walk.last().copied().unwrap();
    while let Some(step) = follow(&field, pos) {
        pos = step;
        walk.push(step);
    }

    let start_connection = (start[0].direction, walk.last().unwrap().direction);

    let crossings: HashSet<_> = walk.into_iter().map(|mov| mov.pos).collect();

    let mut inside = 0;
    for (y, row) in field.into_iter().enumerate() {
        let mut crossings_count = 0;
        for (x, tile) in row.into_iter().enumerate() {
            if crossings.contains(&(x, y)) {
                // count only top crossings!
                match tile {
                    Tile::VerticalPipe => crossings_count += 1,
                    Tile::HorizontalPipe => {}
                    Tile::NorthEastBend => crossings_count += 1,
                    Tile::NorthWestBend => crossings_count += 1,
                    Tile::SouthWestBend => {}
                    Tile::SouthEastBend => {}
                    Tile::Start => {
                        if start_connection.0 == Direction::North
                            || start_connection.1 == Direction::North
                        {
                            crossings_count += 1;
                        }
                    }
                    Tile::Ground => unreachable!(),
                }
            } else if crossings_count % 2 == 1 {
                inside += 1;
            }
        }
    }

    println!("{inside}");

    Ok(())
}
