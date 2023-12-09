use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: PathBuf,
}

enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Key([char; 3]);

impl TryFrom<&str> for Key {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.chars();
        let a = iter.next().ok_or(())?;
        let b = iter.next().ok_or(())?;
        let c = iter.next().ok_or(())?;
        Ok(Self([a, b, c]))
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let Some(Ok(instructions)) = lines.next() else {
        return Ok(());
    };
    let instructions: Vec<_> = instructions
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();
    lines.next(); // skip empty line

    let map: HashMap<Key, [Key; 2]> = lines
        .map(|line| {
            let line = line.unwrap();

            let Some((key, value)) = line.split_once(" = (") else {
                panic!("Invalid format.")
            };
            let Some((left, right)) = value.split_once(", ") else {
                panic!("Invalid format.")
            };

            (
                key.try_into().unwrap(),
                [left.try_into().unwrap(), right.try_into().unwrap()],
            )
        })
        .collect();

    let mut step = 0;
    let mut current_instruction = instructions.iter();
    let mut current = &Key(['A', 'A', 'A']);
    while current.0[0] != 'Z' && current.0[1] != 'Z' && current.0[2] != 'Z' {
        match current_instruction.next() {
            None => current_instruction = instructions.iter(),
            Some(Direction::Left) => {
                current = &map[current][0];
                step += 1;
            }
            Some(Direction::Right) => {
                current = &map[current][1];
                step += 1;
            }
        }
    }

    println!("{step}");

    Ok(())
}
