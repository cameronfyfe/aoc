#[macro_use]
extern crate scan_fmt;
use itertools::Itertools;
use std::{env, fs};

fn find_optimal_happiness(input: &str, with_me: bool) -> i32 {
    let happiness_info = input
        .lines()
        .map(|line| {
            let (person, dir, amt, neighbor) = scan_fmt!(
                line,
                "{} would {} {} happiness units by sitting next to {}.",
                String,
                String,
                i32,
                String
            )
            .expect("Error parsing input.");
            (
                person,
                neighbor,
                match dir.as_str() {
                    "gain" => amt,
                    "lose" => 0 - amt,
                    _ => panic!("Error parsing input."),
                },
            )
        })
        .collect::<Vec<(String, String, i32)>>();
    let mut people = happiness_info
        .iter()
        .map(|(person, _, _)| person.as_str())
        .sorted()
        .dedup()
        .collect::<Vec<&str>>();
    if with_me {
        people.push("me");
    }
    let people = people;
    people
        .iter()
        .permutations(people.len())
        .map(|seating_arrangment| {
            seating_arrangment
                .iter()
                .circular_tuple_windows()
                .map(|(a, b)| {
                    let get_happiness = |person: &str, neighbor: &str| {
                        if person == "me" || neighbor == "me" {
                            0
                        } else {
                            happiness_info
                            .iter()
                            .find(|(p, n, _)| person == p && neighbor == n)
                            .expect("Missing happines data.")
                            .2
                        }
                    };
                    get_happiness(a, b) + get_happiness(b, a)
                })
                .sum()
        })
        .max()
        .expect("No max value.")
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).expect("Error: can't read input file.");

    println!("--- Part 1 ---");
    println!("optimal happiness: {}", find_optimal_happiness(&input, false)); // 733

    println!("--- Part 2 ---");
    println!("optimal happiness: {}", find_optimal_happiness(&input, true));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_example_1() {
        assert_eq!(
            find_optimal_happiness(
                "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."
            , false),
            330
        );
    }
}
