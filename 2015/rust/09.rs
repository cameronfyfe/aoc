// scan_fmt = "0.2"
// itertools = "0.10"

#[macro_use]
extern crate scan_fmt;
use itertools::Itertools;
use std::{env, fs};

fn get_all_distances(input: &str) -> Vec<u32> {
    // Location and Distance vectors
    let mut locations: Vec<String> = Vec::new();
    let mut distances = Vec::new();
    // Populate location and distance vectors
    for line in input.lines() {
        let (loc_a, loc_b, distance) = scan_fmt!(line, "{} to {} = {}", String, String, u32)
            .expect("Error: parsing line failed.");
        // Add new locations to locatinos vector
        if !locations.iter().any(|l| l == &loc_a) {
            locations.push(loc_a.clone());
        }
        if !locations.iter().any(|l| l == &loc_b) {
            locations.push(loc_b.clone());
        }
        // Add distance to distances vector
        distances.push((loc_a, loc_b, distance));
    }
    // Vector of all possible distances
    locations
        .iter()
        .permutations(locations.len())
        .map(|route| {
            route
                .iter()
                .tuple_windows::<(_, _)>()
                .map(|(loc_a, loc_b)| {
                    distances
                        .iter()
                        .find(|(a, b, _)| {
                            (loc_a == &a && loc_b == &b) || (loc_a == &b && loc_b == &a)
                        })
                        .expect("Error: couldn't find distance for 2 locations")
                        .2
                })
                .sum()
        })
        .unique()
        .collect()
}

fn part_1(distances: &Vec<u32>) -> u32 {
    distances
        .iter()
        .min()
        .map(|&d| d)
        .expect("Error: couldn't find min distance")
}

fn part_2(distances: &Vec<u32>) -> u32 {
    distances
        .iter()
        .max()
        .map(|&d| d)
        .expect("Error: couldn't find max distance")
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).expect("Error: can't read input file.");

    let distances = get_all_distances(&input);

    println!("--- Part 1 ---");
    println!("distance: {}", part_1(&distances)); // 141

    println!("--- Part 2 ---");
    println!("distance: {}", part_2(&distances)); // 736
}
