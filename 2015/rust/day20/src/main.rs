use std::{env, fs};


fn all_factors(n: u64) -> Vec<u64> {
    let sqrt = (n as f32).sqrt().ceil() as u64;
    let mut factors = (1..=sqrt).filter(|i| n % i == 0).map(|i| vec![i, n/i]).collect::<Vec<Vec<u64>>>().concat();
    factors.sort();
    factors.dedup();
    factors
}


fn all_factors_within50(n: u64) -> Vec<u64> {
    all_factors(n).iter().filter(|&f| f*50 >= n).map(|&f| f).collect::<Vec<u64>>()
}


fn num_presents(house_num: u64) -> u64 {
    all_factors(house_num).iter().map(|n| n*10).sum()
}


fn num_presents_within50(house_num: u64) -> u64 {
    all_factors_within50(house_num).iter().map(|n| n*11).sum()
}

fn part_1(input: &str) -> u64 {

    let min_presents = input.parse::<u64>().expect("Error parsing input string as u64.");
    let mut max = 0;
    let mut house_num = 1;
    loop {
        let presents = num_presents(house_num);
        max = std::cmp::max(max, presents);

        if house_num % 10000 == 0 {
            println!("House {} got {} presents. max so far: {}/{} = %{}", house_num, presents, max, min_presents, 100*max/min_presents);
        }

        if presents >= min_presents {
            break;
        } else {
            house_num += 1;
        }
    }

    house_num
}


fn part_2(input: &str) -> u64 {

    let min_presents = input.parse::<u64>().expect("Error parsing input string as u64.");
    let mut max = 0;
    let mut house_num = 1;
    loop {
        let presents = num_presents_within50(house_num);
        max = std::cmp::max(max, presents);

        if house_num % 10000 == 0 {
            println!("House {} got {} presents. max so far: {}/{} = %{}", house_num, presents, max, min_presents, 100*max/min_presents);
        }

        if presents >= min_presents {
            break;
        } else {
            house_num += 1;
        }
    }

    house_num
}


fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let input = fs::read_to_string(&args[0]).expect("Error reading input file.");

    let part1 = part_1(&input);
    let part2 = part_2(&input);

    println!("--- Part 1 ---");
    println!("house #: {}", part1); // 831600

    // Part 2 is very slow but I just let it run instead of taking time to optimize it
    println!("--- Part 2 ---");
    println!("house #: {}", part2); // 884520
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_example_1() {
        assert_eq!(num_presents(1), 10);
    }
    #[test]
    fn part_1_example_2() {
        assert_eq!(num_presents(2), 30);
    }
    #[test]
    fn part_1_example_3() {
        assert_eq!(num_presents(3), 40);
    }
    #[test]
    fn part_1_example_4() {
        assert_eq!(num_presents(4), 70);
    }
    #[test]
    fn part_1_example_5() {
        assert_eq!(num_presents(5), 60);
    }
    #[test]
    fn part_1_example_6() {
        assert_eq!(num_presents(6), 120);
    }
    #[test]
    fn part_1_example_7() {
        assert_eq!(num_presents(7), 80);
    }
    #[test]
    fn part_1_example_8() {
        assert_eq!(num_presents(8), 150);
    }
    #[test]
    fn part_1_example_9() {
        assert_eq!(num_presents(9), 130);
    }
}