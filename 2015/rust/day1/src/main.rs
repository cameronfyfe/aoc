use std::{env, fs};

fn part_1(input: &Vec<char>) {
    let mut floor = 0;
    for c in input {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
    }
    println!("Floor: {}", floor);
}

fn part_2(input: &Vec<char>) {
    let mut floor = 0;
    for (i, c) in input.iter().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor == -1 {
            println!("Basement Entry: {}", i + 1);
            return;
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read(&args[1])
        .unwrap()
        .iter()
        .map(|c| *c as char)
        .collect::<Vec<char>>();

    part_1(&input);
    part_2(&input);
}
