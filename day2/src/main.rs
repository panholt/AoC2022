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
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => Move::Unknown,
    }
}

fn their_play_to_mine(theirs: &Move, mine: &str) -> Move {
    match mine {
        "X" => {
            match &theirs {
                Move::Rock => return Move::Scissors,
                Move::Paper => return Move::Rock,
                Move::Scissors => return Move::Paper,
                Move::Unknown => return Move::Unknown,
            };
        }
        "Y" => {
            match &theirs {
                Move::Rock => return Move::Rock,
                Move::Paper => return Move::Paper,
                Move::Scissors => return Move::Scissors,
                Move::Unknown => return Move::Unknown,
        };
    }
       "Z" => {
            match &theirs {
                Move::Rock => return Move::Paper,
                Move::Paper => return Move::Scissors,
                Move::Scissors => return Move::Rock,
                Move::Unknown => return Move::Unknown,
        };
    }
    _ => return Move::Unknown
}
}

fn main() {
    let file = fs::File::open("../inputs/day2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let elems: Vec<&str> = line.split(' ').collect();
        let their_play = play_to_move(elems[0]);
        let my_play = their_play_to_mine(&their_play, elems[1]);
        let round = Round{
            theirs: their_play,
            mine: my_play
        };
        score += round.score();
    };
    println!("Total score is: {}", score);
}
