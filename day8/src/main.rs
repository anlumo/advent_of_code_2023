use std::{
    collections::HashMap,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;
use num::Integer;

#[derive(Parser, Debug)]
struct Args {
    filename: PathBuf,
}

enum Direction {
    Left,
    Right,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Key([char; 3]);

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key[{}{}{}]", self.0[0], self.0[1], self.0[2])
    }
}

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
    let mut current_instruction = instructions.iter().enumerate();
    let start: Vec<_> = map.keys().filter(|key| key.0[2] == 'A').collect();
    let mut current = start.clone();
    // eprintln!("start keys = {current:?}");
    let mut cycles: Vec<_> = start.iter().map(|&key| (vec![(*key, 0)], false)).collect();
    while current.iter().any(|&c| c.0[2] != 'Z') {
        match current_instruction.next() {
            None => {
                current_instruction = instructions.iter().enumerate();
                continue;
            }
            Some((instruction, Direction::Left)) => {
                for (idx, c) in current.iter_mut().enumerate() {
                    *c = &map[c][0];
                    if !cycles[idx].1 {
                        if cycles[idx].0.contains(&(**c, instruction + 1)) {
                            cycles[idx].0.push((**c, instruction + 1));

                            cycles[idx].1 = true;
                            if cycles.iter().all(|&(_, done)| done) {
                                finalize(
                                    &cycles
                                        .into_iter()
                                        .map(|(entry, _)| entry)
                                        .collect::<Vec<_>>(),
                                );
                                return Ok(());
                            }
                        } else {
                            cycles[idx].0.push((**c, instruction + 1));
                        }
                    }
                }
            }
            Some((instruction, Direction::Right)) => {
                for (idx, c) in current.iter_mut().enumerate() {
                    *c = &map[c][1];
                    if !cycles[idx].1 {
                        if cycles[idx].0.contains(&(**c, instruction + 1)) {
                            cycles[idx].0.push((**c, instruction + 1));

                            cycles[idx].1 = true;
                            if cycles.iter().all(|&(_, done)| done) {
                                finalize(
                                    &cycles
                                        .into_iter()
                                        .map(|(entry, _)| entry)
                                        .collect::<Vec<_>>(),
                                );
                                return Ok(());
                            }
                        } else {
                            cycles[idx].0.push((**c, instruction + 1));
                        }
                    }
                }
            }
        }
        step += 1;
    }

    println!("{step}");

    Ok(())
}

fn finalize(cycles: &[Vec<(Key, usize)>]) {
    let mut equation = Vec::new();
    for (cycle_idx, cycle) in cycles.iter().enumerate() {
        let last = cycle.last().unwrap();
        let (_, prefix) = cycle.iter().find(|&entry| *entry == *last).unwrap();
        let (suffix, _) = cycle
            .iter()
            .enumerate()
            .find(|(_, (key, _))| key.0[2] == 'Z')
            .unwrap();
        let cycle_len = cycle.len() - prefix - 1;

        println!(
            "cycle {cycle_idx}: {prefix} + n * {cycle_len} + {}",
            suffix - prefix
        );
        // println!("{:?}", &cycle[..100]);
        equation.push((prefix, cycle_len, suffix - prefix));
    }

    let lcm = equation.iter().fold(1, |prev, &(_, cur, _)| cur.lcm(&prev));
    println!("lcm = {lcm}");
}
