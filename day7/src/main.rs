use std::{
    cmp::Ordering,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    filename: PathBuf,
}

#[derive(Debug)]
#[repr(u8)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq)]
struct Bet {
    cards: [u8; 5],
    bid: usize,
}

impl Bet {
    fn analyze(&self) -> HandType {
        let j_count = self.cards.iter().filter(|&c| *c == 1).count();
        let set: HashSet<_> = self.cards.iter().copied().collect();
        match set.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                let unique: Vec<_> = set.iter().copied().collect();
                match (
                    self.cards.iter().filter(|&c| *c == unique[0]).count(),
                    self.cards.iter().filter(|&c| *c == unique[1]).count(),
                ) {
                    (4, 1) | (1, 4) if j_count == 0 => HandType::FourOfAKind,
                    (4, 1) | (1, 4) => HandType::FiveOfAKind,
                    (3, 2) | (2, 3) if j_count == 0 => HandType::FullHouse,
                    (3, 2) | (2, 3) if j_count == 1 => HandType::FourOfAKind,
                    (3, 2) | (2, 3) => HandType::FiveOfAKind,
                    _ => unreachable!(),
                }
            }
            3 => {
                let unique: Vec<_> = set.iter().copied().collect();
                match (
                    self.cards.iter().filter(|&c| *c == unique[0]).count(),
                    self.cards.iter().filter(|&c| *c == unique[1]).count(),
                    self.cards.iter().filter(|&c| *c == unique[2]).count(),
                ) {
                    (3, 1, 1) | (1, 3, 1) | (1, 1, 3) if j_count == 0 => HandType::ThreeOfAKind,
                    (3, 1, 1) | (1, 3, 1) | (1, 1, 3) => HandType::FourOfAKind,
                    (2, 2, 1) | (2, 1, 2) | (1, 2, 2) if j_count == 0 => HandType::TwoPair,
                    (2, 2, 1) | (2, 1, 2) | (1, 2, 2) if j_count == 1 => HandType::FullHouse,
                    (2, 2, 1) | (2, 1, 2) | (1, 2, 2) => HandType::FourOfAKind,
                    _ => unreachable!(),
                }
            }
            4 => {
                if j_count == 0 {
                    HandType::OnePair
                } else {
                    HandType::ThreeOfAKind
                }
            }
            _ => {
                if j_count == 0 {
                    HandType::HighCard
                } else {
                    HandType::OnePair
                }
            }
        }
    }
}

impl Ord for Bet {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.analyze();
        let other_type = other.analyze();
        let ord = (self_type as u8).cmp(&(other_type as u8));
        if ord == Ordering::Equal {
            self.cards.cmp(&other.cards)
        } else {
            ord
        }
    }
}

impl PartialOrd for Bet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for Bet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Bet {{ cards: {}, bid: {}, type: {:?} }}",
            self.cards
                .iter()
                .map(|&card| match card {
                    1 => 'J',
                    2 => '2',
                    3 => '3',
                    4 => '4',
                    5 => '5',
                    6 => '6',
                    7 => '7',
                    8 => '8',
                    9 => '9',
                    10 => 'T',
                    11 => 'Q',
                    12 => 'K',
                    13 => 'A',
                    _ => '?',
                })
                .collect::<String>(),
            self.bid,
            self.analyze(),
        )
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.filename)?;
    let reader = BufReader::new(file);

    let mut bets: Vec<Bet> = reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let Some((hand, bid)) = line.split_once(' ') else {
                return None;
            };

            Some(Bet {
                cards: hand
                    .chars()
                    .map(|c| match c {
                        'J' => 1,
                        '2' => 2,
                        '3' => 3,
                        '4' => 4,
                        '5' => 5,
                        '6' => 6,
                        '7' => 7,
                        '8' => 8,
                        '9' => 9,
                        'T' => 10,
                        'Q' => 11,
                        'K' => 12,
                        'A' => 13,
                        _ => 0,
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .ok()?,
                bid: bid.parse().ok()?,
            })
        })
        .collect();

    bets.sort();

    let result = bets
        .into_iter()
        .enumerate()
        .map(|(rank, bet)| (rank + 1) * bet.bid)
        .sum::<usize>();

    println!("Total winnings: {result}");

    Ok(())
}
