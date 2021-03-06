#[macro_use]
extern crate scan_fmt;
use std::collections::HashMap;
use std::{env, fs};

struct Sue {
    id: usize,
    properties: HashMap<String, u32>,
}

impl Sue {
    fn from_line(line: &str) -> Sue {
        let first_sc = line
            .chars()
            .position(|c| c == ':')
            .expect("Error parsing line.");
        let id = scan_fmt!(&line[..first_sc], "Sue {}", usize)
            .expect("Error parsing line with Sue info.");
        let mut properties = HashMap::new();
        for item in line[first_sc + 2..].split(", ") {
            let (key, value) =
                scan_fmt!(item, "{}: {}", String, u32).expect("Error parsing property on line.");
            properties.insert(key, value);
        }
        Sue { id, properties }
    }
}

fn ticker_tape() -> HashMap<String, u32> {
    let mut tape = HashMap::new();
    for p in vec![
        ("children", 3 as u32),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .iter()
    {
        tape.insert(p.0.to_string(), p.1);
    }
    tape
}

fn parts_1_and_2(input: &str, part_2: bool) -> Option<usize> {
    let sues = input
        .lines()
        .map(|line| Sue::from_line(line))
        .collect::<Vec<Sue>>();
    let tape = ticker_tape();

    match sues.iter().find(|sue| {
        // True if all properties match ticker tape
        sue.properties
            .iter()
            .map(|p| match tape.get(p.0) {
                Some(val) => if part_2 {
                    match p.0.as_str() {
                        "cats" | "trees" => (p.1 > val),
                        "pomeranians" | "goldfish" => (p.1 < val),
                        _ => (p.1 == val),
                    }
                } else {
                    p.1 == val
                }
                None => false,
            })
            .any(|b| b == false) == false
    }) {
        Some(sue) => Some(sue.id),
        None => None,
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).expect("Error reading input file.");

    println!("--- Part 1 ---");
    println!(
        "Sue number: {}",
        parts_1_and_2(&input, false).expect("Couldn't find Sue match.")
    );

    println!("--- Part 2 ---");
    println!(
        "Sue number: {}",
        parts_1_and_2(&input, true).expect("Couldn't find Sue match.")
    );
}
