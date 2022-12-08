use std::fs;
use std::io;
use std::io::BufRead; 

fn main() {
    let mut elves: Vec<i32> = Vec::new();
    let mut total = 0;
    let file = fs::File::open("../inputs/day1.txt").unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            elves.push(total);
            total = 0;
        } else {
            let number = line.parse::<i32>().unwrap();
            total += number;
        }
    }
    elves.sort();
    elves.reverse();
    println!("Elf with the most calories is {}", &elves[0]);
    let top_three: i32 = elves[0..3].iter().sum();
    println!("The top three elves have {} calories", top_three);
}
