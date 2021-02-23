#[macro_use]
extern crate scan_fmt;
use std::{cmp, env, fs, mem};

enum LightCommand {
    TurnOn,
    TurnOff,
    Toggle,
}

impl LightCommand {
    fn to_string(&self) -> &str {
        match self {
            LightCommand::TurnOn => "turn on",
            LightCommand::TurnOff => "turn off",
            LightCommand::Toggle => "toggle",
        }
    }
    fn from_string(s: &str) -> LightCommand {
        match s.chars().nth(6).unwrap() {
            'n' => LightCommand::TurnOn,
            'f' => LightCommand::TurnOff,
            ' ' => LightCommand::Toggle,
            _ => {
                panic!()
            }
        }
    }
}

fn get_light_cmd(line: &str) -> (LightCommand, usize, usize, usize, usize) {
    let cmd = LightCommand::from_string(line);
    let (x_a, y_a, x_b, y_b) = scan_fmt!(
        line,
        &format!("{} {{}},{{}} through {{}},{{}}", cmd.to_string()),
        usize,
        usize,
        usize,
        usize
    )
    .unwrap();
    let (x1, x2) = min_max(x_a, x_b);
    let (y1, y2) = min_max(y_a, y_b);
    (cmd, x1, x2, y1, y2)
}

fn part_1(input: &str) {
    let mut lights = [[false; 1000]; 1000];
    for line in input.lines() {
        let (cmd, x1, x2, y1, y2) = get_light_cmd(line);
        for x in x1..=x2 {
            for y in y1..=y2 {
                lights[x][y] = match cmd {
                    LightCommand::TurnOn => true,
                    LightCommand::TurnOff => false,
                    LightCommand::Toggle => !lights[x][y],
                };
            }
        }
    }
    let light_strand: &[bool; 1000 * 1000] = unsafe { mem::transmute(&lights) };
    let num_lights_lit = light_strand.iter().filter(|&l| *l == true).count();

    println!("--- Part One ---");
    println!("lights lit: {}", num_lights_lit);
}

fn part_2(input: &str) {
    let mut lights = [[0 as u32; 1000]; 1000];
    for line in input.lines() {
        let (cmd, x1, x2, y1, y2) = get_light_cmd(line);
        for x in x1..=x2 {
            for y in y1..=y2 {
                lights[x][y] = match cmd {
                    LightCommand::TurnOn => lights[x][y] + 1,
                    LightCommand::TurnOff => {
                        if lights[x][y] > 0 {
                            lights[x][y] - 1
                        } else {
                            0
                        }
                    }
                    LightCommand::Toggle => lights[x][y] + 2,
                };
            }
        }
    }
    let light_strand: &[u32; 1000 * 1000] = unsafe { mem::transmute(&lights) };
    let total_brightness = light_strand.iter().sum::<u32>();
    
    println!("--- Part Two ---");
    println!("total brightness: {}", total_brightness);
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).unwrap();

    part_1(&input);
    part_2(&input);
}

fn min_max(a: usize, b: usize) -> (usize, usize) {
    (cmp::min(a, b), cmp::max(a, b))
}
