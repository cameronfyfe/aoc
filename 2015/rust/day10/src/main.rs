use itertools::Itertools;
use std::{env, fs};

// Return look-and-say result for given sequence
fn look_and_say(seq: &str) -> String {
    seq.chars()
        .map(|num| (1, num))
        .coalesce(|a, b| {
            if (a.1) == (b.1) {
                Ok((a.0 + b.0, a.1))
            } else {
                Err((a, b))
            }
        })
        .map(|(count, num)| format!("{}{}", count, num))
        .join("")
}

// Return result of applying look-and-say n times to given sequence
fn look_and_say_n(seq: &str, n: u32) -> String {
    let mut sequence = seq.to_string();
    for _ in 0..n {
        sequence = look_and_say(&sequence);
    }
    sequence
}

fn part_1(input: &str) -> String {
    look_and_say_n(input, 40)
}

fn part_2(input: &str) -> String {
    look_and_say_n(input, 50)
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).expect("Error: can't read input file.");

    println!("--- Part 1 ---");
    println!("sequence length: {}", part_1(&input).len()); // 360154

    println!("--- Part 2 ---");
    println!("sequence length: {}", part_2(&input).len()); // 5103798
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prompt_example_1() {
        assert_eq!(look_and_say("1"), "11");
    }
    #[test]
    fn prompt_example_2() {
        assert_eq!(look_and_say("11"), "21");
    }
    #[test]
    fn prompt_example_3() {
        assert_eq!(look_and_say("21"), "1211");
    }
    #[test]
    fn prompt_example_4() {
        assert_eq!(look_and_say("1211"), "111221");
    }
    #[test]
    fn prompt_example_5() {
        assert_eq!(look_and_say("111221"), "312211");
    }
}
