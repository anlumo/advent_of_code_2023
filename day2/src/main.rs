use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: PathBuf,
    #[clap(short, long)]
    red: usize,
    #[clap(short, long)]
    green: usize,
    #[clap(short, long)]
    blue: usize,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);

    let sum: usize = reader
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let line = line.unwrap();
            let Some((_, def)) = line.split_once(": ") else {
                return None;
            };

            for draw in def.split("; ") {
                for pick in draw.split(", ") {
                    if let Some((count, name)) = pick.split_once(' ') {
                        let count = count.parse::<usize>().unwrap();
                        match name {
                            "red" => {
                                if args.red < count {
                                    return None;
                                }
                            }
                            "green" => {
                                if args.green < count {
                                    return None;
                                }
                            }
                            "blue" => {
                                if args.blue < count {
                                    return None;
                                }
                            }
                            _ => return None,
                        }
                    }
                }
            }

            Some(idx + 1)
        })
        .sum();

    println!("{sum}");
    Ok(())
}
