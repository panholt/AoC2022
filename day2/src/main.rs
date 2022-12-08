use std::fs;
use std::io::{BufRead, BufReader}; 

enum Move {
    Rock,
    Paper,
    Scissors,
    Unknown,
}

enum Result {
    Win,
    Lose,
    Draw,
    Unknown,
}

struct Round {
    theirs: Move,
    mine: Move,
}

impl Round {

    fn find_result(&self) -> Result {
        match (&self.mine, &self.theirs) {
            (Move::Rock, Move::Rock) => return Result::Draw,
            (Move::Rock, Move::Paper) => return Result::Lose,
            (Move::Rock, Move::Scissors) => return Result::Win,
            (Move::Paper, Move::Rock) => return Result::Win,
            (Move::Paper, Move::Paper) => return Result::Draw, 
            (Move::Paper, Move::Scissors) => return Result::Lose,
            (Move::Scissors, Move::Rock) => return Result::Lose,
            (Move::Scissors, Move::Paper) => return Result::Win,
            (Move::Scissors, Move::Scissors) => return Result::Draw,
            _ => return Result::Unknown,
        };
    }

    fn score(&self) -> i32 {
        let mut total = 0;
        let outcome = &self.find_result();
        match outcome {
            Result::Win => total += 6,
            Result::Lose => (),
            Result::Draw => total += 3,
            _ => (),
        };
        match &self.mine {
            Move::Rock => total += 1,
            Move::Paper => total += 2,
            Move::Scissors => total += 3,
            _ => (),
        }
        total
    }

}


fn play_to_move(play: &str) -> Move {
    match play {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => Move::Unknown,
    }
}

fn main() {
    let file = fs::File::open("../inputs/day2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let elems: Vec<&str> = line.split(' ').collect();
        let round = Round{
            theirs: play_to_move(elems[0]),
            mine: play_to_move(elems[1])
        };
        score += round.score();
    };
    println!("Total score is: {}", score);
}
