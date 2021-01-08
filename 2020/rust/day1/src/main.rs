use std::fs;

fn part_1(input: &Vec<u32>) {
    for (x_i, x) in input.iter().enumerate() {
        for y in &input[x_i + 1..input.len()] {
            if x + y == 2020 {
                let answer = x * y;
                println!("{}+{}=2020, {}*{}={}", x, y, x, y, answer);
            }
        }
    }
}

fn part_2(input: &Vec<u32>) {
    for (x_i, x) in input.iter().enumerate() {
        for (y_i, y) in input[x_i + 1..input.len()].iter().enumerate() {
            for z in &input[y_i + 1..input.len()] {
                if x + y + z == 2020 {
                    let answer = x * y * z;
                    println!("{}+{}+{}=2020, {}*{}*{}={}", x, y, z, x, y, z, answer);
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    println!("--- Part 1 ---");
    part_1(&input);

    println!("--- Part 2 ---");
    part_2(&input);
}
