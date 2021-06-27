use std::{env, fs};

fn move_house(loc: &(i32, i32), dir: &char) -> (i32, i32) {
    match dir {
        '^' => (loc.0, loc.1 + 1),
        'v' => (loc.0, loc.1 - 1),
        '>' => (loc.0 + 1, loc.1),
        '<' => (loc.0 - 1, loc.1),
        _ => loc.clone()
    }
}

fn part_1(input: &str) -> usize {
    let mut houses_visited = Vec::new();
    let mut house = (0, 0);
    houses_visited.push(house);

    for direction in input.chars() {
        house = move_house(&house, direction);
        if houses_visited.iter().find(|&&h| h == house).is_none() {
            houses_visited.push(house);
        }
    }

    houses_visited.len()
}

fn part_2(input: &str) -> usize {
    let mut houses_visited = Vec::new();
    let start = (0, 0);
    let mut house_santa = start;
    let mut house_robo_santa = start;
    houses_visited.push(start);

    for (i, direction) in input.chars().enumerate() {
        let house = if i % 2 == 0 {
            &mut house_santa
        } else {
            &mut house_robo_santa
        };
        *house = move_house(house, &direction);
        if houses_visited.iter().find(|&&h| h == *house).is_none() {
            houses_visited.push(*house);
        }
    }

    houses_visited.len()
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).expect("Error: can't read input file.");

    println!("--- Part 1 ---");
    println!("houses visited: {}", part_1(&input));
    println!("--- Part 2 ---");
    println!("houses visited: {}", part_2(&input));
}
