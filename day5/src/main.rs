use std::fs;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;
use regex::Regex;

struct SantasPort {
    lane_1: VecDeque<char>,
    lane_2: VecDeque<char>,
    lane_3: VecDeque<char>,
    lane_4: VecDeque<char>,
    lane_5: VecDeque<char>,
    lane_6: VecDeque<char>,
    lane_7: VecDeque<char>,
    lane_8: VecDeque<char>,
    lane_9: VecDeque<char>,
}

fn main() {
    let file = fs::File::open("../inputs/day5.txt").unwrap();
    let reader = BufReader::new(file);
    let mut finished_init_flag = false;
    let mut port = SantasPort{
        lane_1: VecDeque::new(),
        lane_2: VecDeque::new(),
        lane_3: VecDeque::new(),
        lane_4: VecDeque::new(),
        lane_5: VecDeque::new(),
        lane_6: VecDeque::new(),
        lane_7: VecDeque::new(),
        lane_8: VecDeque::new(),
        lane_9: VecDeque::new(),
    };
    for line in reader.lines() {
        let line = line.unwrap();
        if !finished_init_flag {
            if line.is_empty() {
                finished_init_flag = true;
                continue;
            } else {
            for (i, c) in line.chars().enumerate(){
                match i{
                    1 => {if c.is_ascii_uppercase(){
                        port.lane_1.push_back(c)
                    }},
                    5 => {if c.is_ascii_uppercase(){
                        port.lane_2.push_back(c)
                    }},
                    9 => {if c.is_ascii_uppercase(){
                        port.lane_3.push_back(c)
                    }},
                    13 => {if c.is_ascii_uppercase(){
                        port.lane_4.push_back(c)
                    }},
                    17 => {if c.is_ascii_uppercase(){
                        port.lane_5.push_back(c)
                    }},
                    21 => {if c.is_ascii_uppercase(){
                        port.lane_6.push_back(c)
                    }},
                    25 => {if c.is_ascii_uppercase(){
                        port.lane_7.push_back(c)
                    }},
                    29 => {if c.is_ascii_uppercase(){
                        port.lane_8.push_back(c)
                    }},
                    33 => {if c.is_ascii_uppercase(){
                        port.lane_9.push_back(c)
                    }},
                    _ => (),
                };
            };
            continue;
            };
        };
    // Done setting up, now start making moves
    
    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    println!("{}", line);
    let caps = re.captures(&line).unwrap();
    let count = caps[1].parse::<i32>().unwrap();
    let source_lane = caps[2].parse::<i32>().unwrap();
    let dest_lane: i32 = caps[3].parse::<i32>().unwrap();
    let mut items: VecDeque<char> = VecDeque::new();
    println!("Lane 1: {:?}", port.lane_1);
    println!("Lane 2: {:?}", port.lane_2);
    println!("Lane 3: {:?}", port.lane_3);
    println!("Lane 4: {:?}", port.lane_4);
    println!("Lane 5: {:?}", port.lane_5);
    println!("Lane 6: {:?}", port.lane_6);
    println!("Lane 7: {:?}", port.lane_7);
    println!("Lane 8: {:?}", port.lane_8);
    println!("Lane 9: {:?}", port.lane_9);
    if source_lane == 1{
        items = port.lane_1.drain(..count as usize).collect::<VecDeque<char>>();
    } else if source_lane == 2 {
        items = port.lane_2.drain(..count as usize).collect::<VecDeque<char>>();
    } else if source_lane == 3 {
        items = port.lane_3.drain(..count as usize).collect::<VecDeque<char>>();
    } else if source_lane == 4 {
        items = port.lane_4.drain(..count as usize).collect::<VecDeque<char>>();
    } else if source_lane == 5 {
        items = port.lane_5.drain(..count as usize).collect::<VecDeque<char>>();
    } else if source_lane == 6 {
        items = port.lane_6.drain(..count as usize).collect::<VecDeque<char>>();
    } else if source_lane == 7 {
        items = port.lane_7.drain(..count as usize).collect::<VecDeque<char>>();
    } else if source_lane == 8 {
        items = port.lane_8.drain(..count as usize).collect::<VecDeque<char>>();
    } else if source_lane == 9 {
        items = port.lane_9.drain(..count as usize).collect::<VecDeque<char>>();
    };   
    println!("Items {:?}", items);
    let mut new_items: Vec<char> = Vec::new();
    for char in items{
        new_items.push(char);
    }
    // new_items.reverse();
    // println!("New Items: {:?}", new_items);
    match dest_lane{
        1 => {for char in new_items {port.lane_1.push_front(char);}}
        2 => {for char in new_items {port.lane_2.push_front(char);}}
        3 => {for char in new_items {port.lane_3.push_front(char);}}
        4 => {for char in new_items {port.lane_4.push_front(char);}}
        5 => {for char in new_items {port.lane_5.push_front(char);}}
        6 => {for char in new_items {port.lane_6.push_front(char);}}
        7 => {for char in new_items {port.lane_7.push_front(char);}}
        8 => {for char in new_items {port.lane_8.push_front(char);}}
        9 => {for char in new_items {port.lane_9.push_front(char);}}
        _ => (),
    };
    println!("Lane 1: {:?}", port.lane_1);
    println!("Lane 2: {:?}", port.lane_2);
    println!("Lane 3: {:?}", port.lane_3);
    println!("Lane 4: {:?}", port.lane_4);
    println!("Lane 5: {:?}", port.lane_5);
    println!("Lane 6: {:?}", port.lane_6);
    println!("Lane 7: {:?}", port.lane_7);
    println!("Lane 8: {:?}", port.lane_8);
    println!("Lane 9: {:?}", port.lane_9);
    };
}
