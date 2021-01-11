use std::fs;

fn part_1(input: &Vec<char>) {
    let mut houses_visited = Vec::new();
    let mut house = (0, 0);
    houses_visited.push(house);

    for direction in input {
        match direction {
            '>' => { house.0 += 1; }
            '<' => { house.0 -= 1; }
            '^' => { house.1 += 1; }
            'v' => { house.1 -= 1; }
            _ => {}
        }
        if houses_visited.iter().find(|&&h| h == house).is_none() {
            houses_visited.push(house);
        }
    }

    println!("--- Part 1 ---");
    println!("houses visited: {}", houses_visited.len());
}

fn part_2(input: &Vec<char>) {
    let mut houses_visited = Vec::new();
    let start = (0, 0);
    let mut house_santa = start;
    let mut house_robo_santa = start;
    houses_visited.push(start);

    for (i, direction) in input.iter().enumerate() {
        let house = if i % 2 == 0 {
            &mut house_santa
        } else {
            &mut house_robo_santa
        };
        match direction {
            '>' => { (*house).0 += 1; }
            '<' => { (*house).0 -= 1; }
            '^' => { (*house).1 += 1; }
            'v' => { (*house).1 -= 1; }
            _ => {}
        }
        if houses_visited.iter().find(|&&h| h == *house).is_none() {
            houses_visited.push(*house);
        }
    }

    println!("--- Part 2 ---");
    println!("houses visited: {}", houses_visited.len());
}

fn main() {
    let input = fs::read("input.txt")
        .unwrap()
        .iter()
        .map(|&b| b as char)
        .collect::<Vec<char>>();

    part_1(&input);
    part_2(&input);
}
