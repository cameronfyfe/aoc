use std::cmp;
use std::{env, fs};

fn lights_from_str(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => {
                        panic!("Error: invalid char encountered.");
                    }
                })
                .collect()
        })
        .collect()
}

fn run_step(lights: &mut Vec<Vec<bool>>, corners_stuck_on: bool) {
    let mut changes = Vec::new();
    let h = lights.len();
    let w = lights[0].len();
    let corners = vec![(0, 0), (0, h - 1), (w - 1, 0), (w - 1, h - 1)];
    let stuck_lights = if corners_stuck_on { corners } else { vec![] };
    // Stuck lights
    for (i, j) in stuck_lights.iter() {
        lights[*j][*i] = true;
    }
    // Find all lights that need to switch
    for j in 0..h {
        for i in 0..w {
            // Get index ranges for cell neighbors
            let is = cmp::max(0, i as i32 - 1) as usize;
            let ie = cmp::min(w - 1, i + 1);
            let js = cmp::max(0, j as i32 - 1) as usize;
            let je = cmp::min(h - 1, j + 1);
            // Count neighbors
            let mut lit_neighbs = 0;
            for jj in js..=je {
                for ii in is..=ie {
                    if i == ii && j == jj {
                        continue; // cell itself isn't included in neighbors
                    }
                    if lights[jj][ii] {
                        lit_neighbs += 1;
                    }
                }
            }
            // Check if light needs to switch based on current state and number of neighbors
            if (lights[j][i] && (lit_neighbs < 2 || lit_neighbs > 3))
                || (!lights[j][i] && lit_neighbs == 3)
            {
                changes.push((i, j));
            }
        }
    }
    // Update lights with changes
    for (i, j) in changes {
        // Stuck lights
        if stuck_lights.contains(&(i, j)) {
            continue;
        }
        // Switch light
        lights[j][i] = !lights[j][i];
    }
}

fn part(p: u32, input: &str) -> usize {
    let mut lights = lights_from_str(input);
    for _ in 0..100 {
        run_step(&mut lights, if p == 2 { true } else { false });
    }

    lights
        .iter()
        .map(|row| row.iter().filter(|&&light| light).count())
        .sum()
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).expect("Error: can't read input file.");

    for p in 1..=2 {
        println!("--- Part {} ---", p);
        println!("lights on: {}", part(p, &input)); // 1: 1061, 1006:
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_example_1() {
        let mut lights = lights_from_str(
            ".#.#.#
...##.
#....#
..#...
#.#..#
####..",
        );
        let lights_4steps = lights_from_str(
            "......
......
..##..
..##..
......
......",
        );
        for i in 0..4 {
            run_step(&mut lights, false);
        }
        assert_eq!(lights, lights_4steps);
    }
}
