use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);

    let sum: usize = reader
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            let Some((_, def)) = line.split_once(": ") else {
                return None;
            };

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for draw in def.split("; ") {
                for pick in draw.split(", ") {
                    if let Some((count, name)) = pick.split_once(' ') {
                        let count = count.parse::<usize>().unwrap();
                        match name {
                            "red" => {
                                if max_red < count {
                                    max_red = count;
                                }
                            }
                            "green" => {
                                if max_green < count {
                                    max_green = count;
                                }
                            }
                            "blue" => {
                                if max_blue < count {
                                    max_blue = count;
                                }
                            }
                            _ => return None,
                        }
                    }
                }
            }

            Some(max_red * max_green * max_blue)
        })
        .sum();

    println!("{sum}");
    Ok(())
}
