use std::fs;
use std::io::{BufRead, BufReader}; 

fn score_items(items: Vec<char>) -> i32 {
    let mut total = 0;
    let alpha = vec![ 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    for char in items {
        let position = alpha.iter().position(|&x| x == char);
        match position{
            Some(i) => total += i + 1,
            None => ()
        };
        }
    total.try_into().unwrap()
}

fn main() {
    let file = fs::File::open("../inputs/day3.txt").unwrap();
    let reader = BufReader::new(file);
    let mut common_items: Vec<char> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line_size = line.len();
        let (compartment1, compartment2) = line.split_at(line_size / 2);
        for char in compartment1.chars() {
            if compartment2.contains(char) {
                common_items.push(char);
                break;
            }
        };
    };
    println!("Puzzle 1 solution is: {} !!", score_items(common_items));
}