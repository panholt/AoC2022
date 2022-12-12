use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};


fn main() {
    let file = fs::File::open("../inputs/day7.txt").unwrap();
    let reader = BufReader::new(file);
    let mut file_system: HashMap<String, i32> = HashMap::new();
    let cd_re = Regex::new(r"^\$ cd (.*?)$").unwrap();
    let filesize_re = Regex::new(r"(\d+) .*$").unwrap();
    let mut path: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if cd_re.is_match(&line){
            let captures = cd_re.captures(&line).unwrap();
            let mut directory = captures.get(1).unwrap().as_str();
            if directory == ".."{
                path.pop();
            } else {
                if directory == "/" {
                    directory = "ROOT";
                }
                path.push(String::from(directory));
                let abs_path = path.join("/");
                if ! file_system.contains_key(&abs_path) {
                    file_system.insert(abs_path, 0);
                }
            }
        } else if filesize_re.is_match(&line) {
            let captures = filesize_re.captures(&line).unwrap();
            let size: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
            let mut to_update: Vec<String> = vec![];
            for dir in &path{
                to_update.push(String::from(dir));
                let abs_path = to_update.join("/");
                file_system.entry(abs_path).and_modify(|v| *v += size);
            }
        }
}
let mut count = 0;
for (k,v) in file_system.into_iter(){
    if v < 100000 {
        count += v;
    }
}
println!("Puzzle 1 total: {}", count);

}
