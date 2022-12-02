use std::fs;
use fs::read_to_string;
use crate::RoundResult::{Draw, Loss, Win};
use crate::RPS::{Rock, Paper, Scissors};

enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    fn parse(line: &str) -> RPS {
        match line {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!("Unexpected RPS input")
        }
    }

    fn value(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn result(&self, opponent: &RPS) -> RoundResult {
        match self {
            Rock => match opponent {
                Rock => Draw,
                Paper => Loss,
                Scissors => Win,
            }
            Paper => match opponent {
                Rock => Win,
                Paper => Draw,
                Scissors => Loss,
            }
            Scissors => match opponent {
                Rock => Loss,
                Paper => Win,
                Scissors => Draw,
            }
        }
    }

    fn score(&self, opponent: &RPS) -> u32 {
        self.value() + self.result(opponent).value()
    }
}

enum RoundResult {
    Win,
    Draw,
    Loss
}

impl RoundResult {
    fn parse(s: &str) -> RoundResult {
        match s {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Unexpected RoundResult parse input: {}", s)
        }
    }

    fn value(&self) -> u32 {
        match self {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }

    fn required_move(&self, opponent: &RPS) -> RPS {
        match opponent {
            Rock => match self {
                Win => Paper,
                Draw => Rock,
                Loss => Scissors,
            }
            Paper => match self {
                Win => Scissors,
                Draw => Paper,
                Loss => Rock,
            }
            Scissors => match self {
                Win => Rock,
                Draw => Scissors,
                Loss => Paper,
            }
        }
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("Could not read input file");

    let mut pt1_total = 0;
    let mut pt2_total = 0;
    for line in input.lines() {
        let mut splits = line.splitn(2, " ");
        let opp_move = RPS::parse(splits.next().unwrap());
        let xyz = splits.next().unwrap();

        let own_move_pt1 = RPS::parse(xyz);

        let round_result = RoundResult::parse(xyz);
        let own_move_pt2 = round_result.required_move(&opp_move);

        pt1_total += own_move_pt1.score(&opp_move);
        pt2_total += own_move_pt2.score(&opp_move);
    }
    
    println!("Part 1: {}", pt1_total);
    println!("Part 2: {}", pt2_total);
}
