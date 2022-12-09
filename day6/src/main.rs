use std::{fs, collections::HashSet};


fn find_proto_mark(input: &str) -> i32 {
    let mut counter: i32 = 13;
    let input_len = input.len().try_into().unwrap();
    let mut windows = input.as_bytes().windows(14);
    while counter <= input_len {
        let next = windows.next().unwrap();
        counter += 1;
        if is_unique(next){
            return counter
        }
    }
    counter
}

fn is_unique(input: &[u8]) -> bool {
    let input_len = input.len();
    let mut set = HashSet::new();
    for item in input{
        set.insert(item);
    }
    set.len() == input_len
}
fn main() {
    let input = fs::read_to_string("../inputs/day6.txt").unwrap();
    let pos = find_proto_mark(&input);
    println!("{}", pos)
}
