use std::fs;

fn part_1(input: &Vec<Vec<u32>>) {
    let mut amount_of_paper = 0 as u32;

    for present in input {
        let mut sides = present.clone();
        sides.sort();
        let mut sides = sides.iter();
        let h = sides.next().unwrap();
        let w = sides.next().unwrap();
        let l = sides.next().unwrap();
        amount_of_paper += 2*l*w + 2*l*h + 3*w*h;
    }

    println!("--- Part 1 ---");
    println!("paper: {}", amount_of_paper);
}

fn part_2(input: &Vec<Vec<u32>>) {
    let mut amount_of_ribbon = 0 as u32;

    for present in input {
        let mut sides = present.clone();
        sides.sort();
        let mut sides = sides.iter();
        let h = sides.next().unwrap();
        let w = sides.next().unwrap();
        let l = sides.next().unwrap();
        amount_of_ribbon += 2*(w+h) + l*w*h;
    }

    println!("--- Part 2 ---");
    println!("ribbon: {}", amount_of_ribbon);
}
fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split('x')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    part_1(&input);
    part_2(&input);
}
