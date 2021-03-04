use std::{env, fs};
#[macro_use]
extern crate scan_fmt;

#[derive(Clone)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: u32,
}

impl Ingredient {
    fn from_str(str: &str) -> Ingredient {
        let (_, capacity, durability, flavor, texture, calories) = scan_fmt!(
            str,
            "{}: capacity {}, durability {}, flavor {}, texture {}, calories {}",
            String,
            i32,
            i32,
            i32,
            i32,
            u32
        )
        .expect("Error parsing ingredient line.");
        Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
    fn vec_from_lines(lines: &str) -> Vec<Ingredient> {
        lines
            .lines()
            .map(|line| Ingredient::from_str(line))
            .collect()
    }
    fn get_calories(ingredients: &Vec<(u8, Ingredient)>) -> u32 {
        if ingredients.iter().map(|(percent, _)| *percent).sum::<u8>() != 100 {
            panic!("Ingredients don't add up to 100.");
        }
        ingredients
            .iter()
            .map(|(p, i)| *p as u32 * i.calories)
            .sum()
    }
    fn get_score(ingredients: &Vec<(u8, Ingredient)>) -> i64 {
        if ingredients.iter().map(|(percent, _)| *percent).sum::<u8>() != 100 {
            panic!("Ingredients don't add up to 100.");
        }
        let capacity = ingredients
            .iter()
            .map(|(p, i)| *p as i32 * i.capacity)
            .sum::<i32>() as i64;
        let durability = ingredients
            .iter()
            .map(|(p, i)| *p as i32 * i.durability)
            .sum::<i32>() as i64;
        let flavor = ingredients
            .iter()
            .map(|(p, i)| *p as i32 * i.flavor)
            .sum::<i32>() as i64;
        let texture = ingredients
            .iter()
            .map(|(p, i)| *p as i32 * i.texture)
            .sum::<i32>() as i64;

        if capacity >= 0 && durability >= 0 && flavor >= 0 && texture >= 0 {
            capacity * durability * flavor * texture
        } else {
            0
        }
    }
    fn get_max_score(ingredients: &Vec<Ingredient>, calories: Option<u32>) -> i64 {
        let n = ingredients.len();
        let mut amounts_list = Vec::new() as Vec<Vec<u8>>;
        let mut buf = vec![0 as u8; n];
        // Create every set of n positive integers adding up to 100 (including different permuatations)
        'base: loop {
            // If numbers add up to 100:
            if buf.iter().sum::<u8>() == 100 {
                // 1. Add number set to list
                amounts_list.push(buf.clone());
                // 2. Reset first non-zero number and increment next number
                let i = buf
                    .iter()
                    .position(|&num| num != 0)
                    .expect("Invalid algorithm scenario.");
                match buf.get_mut(i + 1) {
                    Some(num) => {
                        *num += 1;
                        buf[i] = 0;
                    }
                    None => {
                        // This is the last possible set, so stop
                        break 'base;
                    }
                }
            // If numbers don't add up to 100, increment first number
            } else {
                buf[0] += 1;
            }
        }
        // Find max score of all the generated combinations
        // Loop manually instead of .max() on map to avoid storing all scores in RAM at once
        let mut max_score = -1 as i64;
        for amounts in amounts_list {
            let recipe = amounts
                .into_iter()
                .zip(ingredients.clone().into_iter())
                .collect::<Vec<(u8, Ingredient)>>();
            if calories.is_none() || Some(Ingredient::get_calories(&recipe)) == calories {
                max_score = std::cmp::max(
                    max_score, Ingredient::get_score(&recipe) 
                );
            }
        }
        max_score
    }
}

fn part_1(input: &str) -> i64 {
    Ingredient::get_max_score(&Ingredient::vec_from_lines(input), None)
}

fn part_2(input: &str) -> i64 {
    Ingredient::get_max_score(&Ingredient::vec_from_lines(input), Some(500))
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).expect("Error reading input file.");

    println!("--- Part 1 ---");
    println!("max score: {}", part_1(&input)); // 18965440

    println!("--- Part 2 ---");
    println!("max score w/ 500 cals: {}", part_2(&input)); // 18965440
}

#[cfg(test)]
mod tests {
    use super::*;
    fn butterscotch() -> Ingredient {
        Ingredient::from_str(
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
        )
    }
    fn cinnamon() -> Ingredient {
        Ingredient::from_str(
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        )
    }
    #[test]
    fn part_1_example_1() {
        assert_eq!(
            Ingredient::get_score(&vec![(44, butterscotch()), (56, cinnamon())]),
            62842880
        );
    }
    #[test]
    fn part_1_example_2() {
        assert_eq!(
            Ingredient::get_max_score(&vec![butterscotch(), cinnamon()], None),
            62842880
        );
    }
    #[test]
    fn part_2_example_1() {
        assert_eq!(
            Ingredient::get_max_score(&vec![butterscotch(), cinnamon()], Some(500)),
            57600000
        );
    }
}
