// md5 = "0.7.0"

use std::{env, fs};
use md5;

fn part_1(input: &str) -> u32 {
    let mut i = 1 as u32;
    loop {
        let hash = md5::compute(format!("{}{}", input, i).chars().map(|c| c as u8).collect::<Vec<u8>>());
        if hash[0..2] == [0, 0] && hash[2] & 0b11110000 == 0 {
            println!("--- Part 1 ---");
            println!("answer: {}", i);
            break;
        }
        i += 1;
    }
    i
}

fn part_2(input: &str, start: u32) {
    let mut i = start;
    loop {
        let hash = md5::compute(format!("{}{}", input, i).chars().map(|c| c as u8).collect::<Vec<u8>>());
        if hash[0..3] == [0, 0, 0] {
            println!("--- Part 2 ---");
            println!("answer: {}", i);
            break;
        }
        i += 1;
    } 
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).unwrap().chars().filter(|c| !c.is_whitespace()).collect::<String>();

    part_2(&input, part_1(&input));
}
