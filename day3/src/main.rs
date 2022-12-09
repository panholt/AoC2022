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

struct GroupRutsack {
    line1: String,
    line2: String,
    line3: String,
}

impl GroupRutsack {

    fn find_common_item(self) -> char {
        for char in self.line1.chars() {
            if self.line2.contains(char) && self.line3.contains(char) {
                return char;
            };
        };
        return ' '
    }
}

fn main() {
    let file = fs::File::open("../inputs/day3.txt").unwrap();
    let reader = BufReader::new(file);
    let mut groups: Vec<GroupRutsack> = Vec::new();
    let mut line_counter = 1;
    let mut line_container: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        match line_counter {
            1 => {line_container.push(line); line_counter += 1},
            2 => {line_container.push(line); line_counter += 1},
            3 => {line_container.push(line); 
                    let group = GroupRutsack{
                    line3: line_container.pop().unwrap(),
                    line2: line_container.pop().unwrap(),
                    line1: line_container.pop().unwrap(),
                };
                groups.push(group);
                line_counter = 1;
            },
            _ => (),
        };
    };
    let mut common_items: Vec<char> = Vec::new();
    for group in groups {
        common_items.push(group.find_common_item());
    }
    println!("Puzzle 2 solution is: {} !!", score_items(common_items));
}