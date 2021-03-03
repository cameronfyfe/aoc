#[macro_use]
extern crate scan_fmt;
use std::collections::HashMap;
use std::{env, fs};

struct Reindeer {
    speed: u32,    // km/s
    duration: u32, // seconds
    rest: u32,     // seconds
}

impl Reindeer {
    fn distance(&self, t: &u32) -> u32 {
        let (v, t_d, t_cycle) = (self.speed, self.duration, self.duration + self.rest);
        v * ((t_d * (t / t_cycle)) + std::cmp::min(t_d, t % t_cycle))
    }
}

fn reindeer_from_line(line: &str) -> Reindeer {
    let (_, speed, duration, rest) = scan_fmt!(
        line,
        "{} can fly {} km/s for {} seconds, but then must rest for {} seconds.",
        String,
        u32,
        u32,
        u32
    )
    .expect("Error parsing reinder line.");
    Reindeer {
        speed,
        duration,
        rest,
    }
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| reindeer_from_line(line).distance(&2503))
        .max()
        .expect("Error: no max found.")
}

fn santas_scoring(time: u32, reindeer: Vec<Reindeer>) -> u32 {
    let mut score = HashMap::new();

    // Determine winners at the end of each second and add 1 to each of their score
    for t in 1..=time {
        let positions = reindeer.iter().enumerate().map(|r| (r.0, r.1.distance(&t)));
        let leading_position = positions
            .clone()
            .map(|p| p.1)
            .max()
            .expect("Error: max not found.");
        let winners = positions.filter(|p| p.1 == leading_position);
        for winner in winners {
            *score.entry(winner.0).or_insert(0) += 1;
        }
    }

    *score.values().max().expect("Error: max not found.")
}

fn part_2(input: &str) -> u32 {
    santas_scoring(
        2503,
        input
            .lines()
            .map(|line| reindeer_from_line(line))
            .collect::<Vec<Reindeer>>(),
    )
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).expect("Error: can't read input file.");

    println!("--- Part 1 ---");
    println!("winner's distance: {}", part_1(&input)); // 2640

    println!("--- Part 2 ---");
    println!("winner's score: {}", part_2(&input)); // 998
}

#[cfg(test)]
mod tests {
    use super::*;
    fn comet() -> Reindeer {
        reindeer_from_line(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
        )
    }
    fn dancer() -> Reindeer {
        reindeer_from_line(
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        )
    }
    #[test]
    fn part_1_example_1() {
        assert_eq!(comet().distance(&1), 14);
        assert_eq!(dancer().distance(&1), 16);
    }
    #[test]
    fn part_1_example_2() {
        assert_eq!(comet().distance(&10), 140);
        assert_eq!(dancer().distance(&10), 160);
    }
    #[test]
    fn part_1_example_3() {
        assert_eq!(comet().distance(&11), 140);
        assert_eq!(dancer().distance(&11), 176);
    }
    #[test]
    fn part_1_example_4() {
        assert_eq!(comet().distance(&12), 140);
        assert_eq!(dancer().distance(&12), 176);
    }
    #[test]
    fn part_1_example_5() {
        assert_eq!(comet().distance(&1000), 1120);
        assert_eq!(dancer().distance(&1000), 1056);
    }
    #[test]
    fn part_2_example_1() {
        assert_eq!(santas_scoring(1000, vec![comet(), dancer()]), 689);
    }
}
