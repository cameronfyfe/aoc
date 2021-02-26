#[macro_use]
extern crate scan_fmt;
use serde::Serialize;
use std::collections::HashMap;
use std::{env, fs};

#[derive(Serialize, Debug)]
enum WireInput {
    Literal(u16),
    Wire(String),
}

impl WireInput {
    fn new(s: &str) -> WireInput {
        match s.parse::<u16>() {
            Ok(n) => WireInput::Literal(n),
            _ => WireInput::Wire(s.to_string()),
        }
    }
    fn value(&self, known_wires: &HashMap<String, u16>) -> Option<u16> {
        match &self {
            WireInput::Literal(n) => Some(*n),
            WireInput::Wire(a) => known_wires.get(a).map(|&n| n),
        }
    }
}

#[derive(Serialize, Debug)]
enum Logic {
    BUF(WireInput),
    NOT(WireInput),
    AND(WireInput, WireInput),
    OR(WireInput, WireInput),
    LSHIFT(WireInput, u8),
    RSHIFT(WireInput, u8),
}

#[derive(Serialize, Debug)]
struct Circuit {
    wire: String,
    logic: Logic,
}

impl Circuit {
    fn solve(&self, known_wires: &HashMap<String, u16>) -> Option<u16> {
        match &self.logic {
            Logic::BUF(a) => a.value(known_wires),
            Logic::NOT(a) => a.value(known_wires).map(|n| !n),
            Logic::AND(a, b) => match (a.value(known_wires), b.value(known_wires)) {
                (Some(n), Some(m)) => Some(n & m),
                _ => None,
            },
            Logic::OR(a, b) => match (a.value(known_wires), b.value(known_wires)) {
                (Some(n), Some(m)) => Some(n | m),
                _ => None,
            },
            Logic::LSHIFT(a, s) => a.value(known_wires).map(|n| n << s),
            Logic::RSHIFT(a, s) => a.value(known_wires).map(|n| n >> s),
        }
    }
}

fn get_circuits(input: &str) -> Vec<Circuit> {
    input
        .lines()
        .map(|line| {
            let line_logic = line.split(" -> ").collect::<Vec<&str>>();
            if line_logic.len() != 2 {
                panic!("Error in line logic syntax. (expecting <logic> -> <wire>)");
            }

            let (logic, wire) = (line_logic[0], line_logic[1]);

            Circuit {
            wire: wire.to_string(),
            logic:
                // AND
                if logic.contains("AND") {
                    let (a, b) = scan_fmt!(logic, "{} AND {}", String, String)
                        .expect("Error parsing AND logic.");
                    Logic::AND(WireInput::new(&a), WireInput::new(&b))
                // OR
                } else if logic.contains("OR") {
                    let (a, b) = scan_fmt!(logic, "{} OR {}", String, String)
                        .expect("Error parsing OR logic.");
                    Logic::OR(WireInput::new(&a), WireInput::new(&b))
                // NOT
                } else if logic.contains("NOT") {
                    let a = scan_fmt!(logic, "NOT {}", String)
                        .expect("Error parsing OR logic.");
                    Logic::NOT(WireInput::new(&a))
                // LSHIFT
                } else if logic.contains("LSHIFT") {
                    let (a, n) = scan_fmt!(logic, "{} LSHIFT {}", String, u8)
                        .expect("Error parsing LSHIFT logic.");
                    Logic::LSHIFT(WireInput::new(&a), n)
                // RSHIFT
                } else if logic.contains("RSHIFT") {
                    let (a, n) = scan_fmt!(logic, "{} RSHIFT {}", String, u8)
                        .expect("Error parsing RSHIFT logic.");
                    Logic::RSHIFT(WireInput::new(&a), n)
                // BUF
                } else {
                    Logic::BUF(WireInput::new(&logic))
                }
            }
        })
        .collect::<Vec<Circuit>>()
}

fn solve_circuits(mut circuits: Vec<Circuit>) -> HashMap<String, u16> {
    let mut solved_circuits = HashMap::new();
    while circuits.len() > 0 {
        circuits.retain(|circuit| match circuit.solve(&solved_circuits) {
            Some(n) => {
                solved_circuits.insert(circuit.wire.clone(), n);
                false
            }
            None => true,
        });
    }
    solved_circuits
}

fn part_1(input: &str) -> u16 {
    let circuits = get_circuits(input);
    let solved_circuits = solve_circuits(circuits);

    *solved_circuits.get("a").expect("Error: wire 'a' unsolved.")
}

fn part_2(input: &str, b_new: u16) -> u16 {
    let mut circuits = get_circuits(input);

    // Change 'b' to value of 'a' from part 1
    let i = circuits
        .iter()
        .position(|c| c.wire == "b")
        .expect("Error: couldn't find 'b'.");
    circuits[i] = Circuit {
        wire: "b".to_string(),
        logic: Logic::BUF(WireInput::new(&b_new.to_string())),
    };

    let solved_circuits = solve_circuits(circuits);

    *solved_circuits.get("a").expect("Error: wire 'a' unsolved.")
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).unwrap();

    println!("--- Part 1 ---");
    let a1 = part_1(&input);
    println!("a: {}", a1); // 16076

    println!("--- Part 2 ---");
    let a2 = part_2(&input, a1);
    println!("a: {}", a2); // 2797
}
