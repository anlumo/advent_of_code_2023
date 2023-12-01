use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Parser)]
struct Args {
    filename: PathBuf,
}

const NUMBER_NAMES: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);

    let sum: usize = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let first_txt = NUMBER_NAMES
                .iter()
                .enumerate()
                .filter_map(|(number, name)| line.find(name).map(|pos| (number + 1, pos)))
                .min_by(|(_, apos), (_, bpos)| apos.cmp(bpos));
            let first_num = line
                .find(NUMBERS)
                .map(|pos| (line.as_bytes()[pos] - b'0', pos));
            let first = if let (Some((txt_num, txt_pos)), Some((num_num, num_pos))) =
                (first_txt, first_num)
            {
                if txt_pos < num_pos {
                    txt_num
                } else {
                    num_num as usize
                }
            } else if let Some((txt_num, _)) = first_txt {
                txt_num
            } else if let Some((num_num, _)) = first_num {
                num_num as usize
            } else {
                panic!("Line doesn't contain any numbers!")
            };
            let last_txt = NUMBER_NAMES
                .iter()
                .enumerate()
                .filter_map(|(number, name)| line.rfind(name).map(|pos| (number + 1, pos)))
                .max_by(|(_, apos), (_, bpos)| apos.cmp(bpos));
            let last_num = line
                .rfind(NUMBERS)
                .map(|pos| (line.as_bytes()[pos] - b'0', pos));
            let last = if let (Some((txt_num, txt_pos)), Some((num_num, num_pos))) =
                (last_txt, last_num)
            {
                if txt_pos > num_pos {
                    txt_num
                } else {
                    num_num as usize
                }
            } else if let Some((txt_num, _)) = last_txt {
                txt_num
            } else if let Some((num_num, _)) = last_num {
                num_num as usize
            } else {
                panic!("Line doesn't contain any numbers!")
            };
            format!("{first}{last}").parse::<usize>().unwrap()
        })
        .sum();

    println!("{sum}");

    Ok(())
}
