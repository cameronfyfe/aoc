use std::{env, fs};
#[derive(Debug)]
struct Machine {
    replacements: Vec<(String, String)>,
}

fn parse_puzzle_input(input: &str) -> (Machine, String) {
    (
        // Machine with replacements list
        Machine {
            replacements: input
                .lines()
                .rev()
                .skip(2)
                .map(|line| {
                    let r = line.split(" => ").collect::<Vec<&str>>();
                    if r.len() == 2 {
                        (r[0].to_string(), r[1].to_string())
                    } else {
                        panic!("Error: parsing replacement list.");
                    }
                })
                .collect::<Vec<(String, String)>>(),
        },
        // Medicine molecule
        input
            .lines()
            .last()
            .expect("Error: problem parsing input.")
            .to_string(),
    )
}

fn part_1(input: &str) -> usize {
    let (machine, medicine_molecule) = parse_puzzle_input(input);

    let mut molecules = machine
        .replacements
        .iter()
        .map(|r| {
            let pieces = medicine_molecule.split(&r.0).collect::<Vec<&str>>();
            let mut molecules = Vec::new();
            for i in 1..pieces.len() {
                let mut molecule = pieces[0].to_string();
                for j in 1..pieces.len() {
                    molecule.push_str(if i == j { &r.1 } else { &r.0 });
                    molecule.push_str(pieces[j]);
                }
                molecules.push(molecule);
            }
            molecules
        })
        .collect::<Vec<Vec<String>>>()
        .concat();

    molecules.sort();
    molecules.dedup();
    molecules.len()
}

fn part_2(input: &str) -> usize {
    let (mut machine, mut medicine_molecule) = parse_puzzle_input(input);

    // Sort replacements in order of largest length change
    machine.replacements.sort_by(|a, b| (b.1.len()-b.0.len()).cmp(&(a.1.len()-a.0.len())));

    let mut count = 0;
    while medicine_molecule != "e" {
        // Try applying replacements in order of largest length change
        // break after any replacements so if new mathces for larget reductions are createt those can be used
        for r in &machine.replacements {
            let num_r = medicine_molecule.matches(&r.1).count();
            if num_r > 0 {
                medicine_molecule = medicine_molecule.replace(&r.1, &r.0);
                count += num_r;
                break;
            }
        }
    }
    
    count
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).expect("Error: can't read input file.");

    println!("--- Part 1 ---");
    println!("# of molecules: {}", part_1(&input));

    println!("--- Part 2 ---");
    println!("# of steps: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_example() {
        let puzzle_input = "H => HO
H => OH
O => HH

HOH";
        assert_eq!(part_1(&puzzle_input), 4);
    }
    #[test]
    fn part_2_example() {
        let puzzle_input = "e => H
e => O
H => HO
H => OH
O => HH

HOH";
        assert_eq!(part_2(&puzzle_input), 3);
    }
}
