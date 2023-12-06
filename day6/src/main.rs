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
    let mut lines = reader.lines();

    let line1 = lines
        .next()
        .expect("time line missing")
        .expect("line 1 is not text");
    let line2 = lines
        .next()
        .expect("distance line missing")
        .expect("line 2 is not text");

    let Some((_, time)) = line1.split_once(':') else {
        panic!("line 1 doesn't have two parts");
    };

    let time = time.replace(' ', "");
    let time = time.parse::<usize>().unwrap();

    let Some((_, distance)) = line2.split_once(':') else {
        panic!("line 2 doesn't have two parts");
    };

    let distance = distance.replace(' ', "");
    let distance = distance.parse::<usize>().unwrap();

    let result = (0..time)
        .map(|acceleration| (acceleration + 1) * (time - acceleration - 1))
        .filter(|&my_distance| my_distance > distance)
        .count();

    println!("{result}");

    Ok(())
}
