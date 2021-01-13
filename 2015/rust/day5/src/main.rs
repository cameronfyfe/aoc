use std::{env, fs};

fn is_nice_alpha(s: &str) -> bool {
    // At least 3 vowels
    s.chars().filter(|c| "aeiou".chars().collect::<Vec<_>>().contains(c)).count() >= 3
    // At least one letter that appears twice in a row
    && (|| {
        let mut i = s.chars().peekable();
        loop {
            let c = match i.next() {
                Some(c) => c,
                None => {
                    break false
                }
            };
            match i.peek() {
                Some(c_next) => {
                    if c == *c_next {
                        break true;
                    }
                }
                None => {
                    break false;
                }
            }
        }
    })()
    // Does not contain specified pairs
    && (|| {
        for pair in vec!["ab", "cd", "pq", "xy"] {
            if s.contains(pair) {
                return false;
            }
        }
        true
    })()
}

fn part_1(input: &str) {
    println!("--- Part One ---");
    println!(
        "nice: {}",
        input.lines().filter(|l| is_nice_alpha(l)).count()
    );
}

fn is_nice_beta(s: &str) -> bool {
    // Contains duplicate non-overlapping pairs
    (|| {
        for i in 0..s.len()-2 {
            let pair = &s[i..i+2];
            if s[..i].contains(pair) || s[i+2..].contains(pair) {
                return true;
            }
        }
        false
    })()
    // Contains duplicate letter with letter in between
    && (|| {
        let s = s.chars().collect::<Vec<_>>();
        for i in 0..s.len()-2 {
            if s[i] == s[i+2] {
                return true
            }
        }
        false
    })()
}

fn part_2(input: &str) {
    println!("--- Part Two ---");
    println!(
        "nice: {}",
        input.lines().filter(|l| is_nice_beta(l)).count()
    );
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).unwrap();

    part_1(&input);
    part_2(&input);
}
