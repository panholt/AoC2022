use std::fs;


fn find_proto_mark(input: &str) -> i32 {
    let mut counter: i32 = 3;
    let input_len = input.len().try_into().unwrap();
    let mut windows = input.as_bytes().windows(4);
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
    input[0] != input[1] &&
    input[0] != input[2] &&
    input[0] != input[3] &&
    input[1] != input[2] &&
    input[1] != input[3] &&
    input[2] != input[3] 
}
fn main() {
    let input = fs::read_to_string("../inputs/day6.txt").unwrap();
    let pos = find_proto_mark(&input);
    println!("{}", pos)
}
