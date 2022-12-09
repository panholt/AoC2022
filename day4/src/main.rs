use std::fs;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Debug)]
struct AssignmentPair {
    range_1: SectionRange,
    range_2: SectionRange,
}

impl AssignmentPair{
    fn fully_contained(self) -> bool {
        (self.range_1.low <= self.range_2.low && 
        self.range_1.high >= self.range_2.high) ||
        (self.range_2.low <= self.range_1.low && 
        self.range_2.high >= self.range_1.high)
    }
}
#[derive(Debug)]
struct SectionRange {
    low: i32,
    high: i32,
}

fn main() {
    let file = fs::File::open("../inputs/day4.txt").unwrap();
    let reader = BufReader::new(file);
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut assignment_pairs: Vec<AssignmentPair> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        for cap in re.captures_iter(&line) {
            assignment_pairs.push(AssignmentPair { 
                                  range_1: SectionRange { low: cap[1].parse::<i32>().unwrap(), 
                                                          high: cap[2].parse::<i32>().unwrap(),},     
                                  range_2: SectionRange { low: cap[3].parse::<i32>().unwrap(), 
                                                           high: cap[4].parse::<i32>().unwrap(),} 
                                  }
                                )
        }

    };
    let mut counter = 0;
    for assignment in assignment_pairs{
        if assignment.fully_contained() {
            counter += 1;
        }
    }
    println!("Day 4 Puzzle 1: {}", counter);
}
