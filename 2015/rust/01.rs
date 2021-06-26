use std::{env, fs};

fn move_floor(c: &char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

fn part_1(input: &str) -> i32 {
    input.chars().map(|c| move_floor(&c)).sum()
}

fn part_2(input: &str) -> Option<usize> {
    input
        .chars()
        .map(|c| move_floor(&c))
        .scan(0, |floor, m| {
            *floor += m;
            Some(*floor)
        })
        .position(|floor| floor == -1)
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).expect("Error: can't read input file.");

    println!("--- Part 1 ---");
    println!("Floor: {}", part_1(&input));
    println!("--- Part 2 ---");
    println!("Basement Entry: {:?}", part_2(&input));
}
