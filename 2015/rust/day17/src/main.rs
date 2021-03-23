use std::{env, fs};

const NUM_LITERS: u32 = 150;

fn all_combos(input: &str) -> Vec<Vec<u32>> {
    let containers = input
        .lines()
        .map(|s| {
            s.parse::<u32>()
                .expect("Error: can't parse number from input.")
        })
        .collect::<Vec<u32>>();
    let mut container_combos = Vec::new();
    for n in 1..=1 << containers.len() {
        let mut combo = Vec::new();
        for (i, c) in containers.iter().enumerate() {
            if n & (1 << i) != 0 {
                combo.push(*c);
            }
        }
        container_combos.push(combo);
    }
    container_combos
}

fn part_1(input: &str) -> usize {
    let container_combos = all_combos(input);

    container_combos
        .iter()
        .filter(|combo| combo.iter().sum::<u32>() == NUM_LITERS)
        .count()
}

fn part_2(input: &str) -> usize {
    let container_combos = all_combos(input);

    let mut num_combos = 0;
    let mut min_containers = container_combos.len(); // guranteed to be more than max num of containers
    for combo in container_combos {
        if combo.iter().sum::<u32>() == NUM_LITERS {
            let num_containers = combo.len();
            if num_containers < min_containers {
                min_containers = num_containers;
                num_combos = 1;
            } else if combo.len() == min_containers {
                num_combos += 1
            }
        }
    }
    num_combos
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).expect("Error: can't read input file.");

    println!("--- Part 1 ---");
    println!("num combos: {}", part_1(&input)); // 1638

    println!("--- Part 2 ---");
    println!("num combos: {}", part_2(&input)); // 17
}
