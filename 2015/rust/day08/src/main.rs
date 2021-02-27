use std::{env, fs};

fn part_1(input: &str) -> usize {
    let mut num_extra_chars = 0;
    for line in input.lines() {
        let mut escaped = false;
        for c in line.chars() {
            match (escaped, c) {
                (false, '\"') => {
                    num_extra_chars += 1;
                }
                (false, '\\') => {
                    escaped = true;
                }
                (true, '\\') => {
                    escaped = false;
                    num_extra_chars += 1;
                }
                (true, '\"') => {
                    escaped = false;
                    num_extra_chars += 1;
                }
                (true, 'x') => {
                    escaped = false;
                    num_extra_chars += 3;
                }
                (_, _) => {}
            }
        }
    }
    num_extra_chars
}

fn part_2(input: &str) -> usize {
    let mut num_extra_chars_encoded = 0;
    for line in input.lines() {
        num_extra_chars_encoded += 2;
        for c in line.chars() {
            if "\"\\".contains(c) {
                num_extra_chars_encoded += 1;
            }
        }
    }
    num_extra_chars_encoded
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).expect("Error: can't read input file.");

    println!("--- Part 1 ---");
    println!("# of extra string literal chars: {}", part_1(&input)); // 1371

    println!("--- Part 2 ---");
    println!("# of string literal chars to encode: {}", part_2(&input)); // 2117
}
