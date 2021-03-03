use serde_json::Value;
use std::{env, fs};

fn balance_book(input: &str) -> i64 {
    let json = serde_json::from_str::<Value>(input).expect("Error: Invalid JSON input.");
    process_value(&json, false)
}

fn balance_book_with_double_red(input: &str) -> i64 {
    let json = serde_json::from_str::<Value>(input).expect("Error: Invalid JSON input.");
    process_value(&json, true)
}

fn process_value(value: &Value, double_red: bool) -> i64 {
    match value {
        Value::Number(num) => num.as_i64().expect("Error: number with more than 64 bits"),
        Value::Array(arr) => arr.iter().map(|v| process_value(v, double_red)).sum(),
        Value::Object(obj) => {
            let sum = obj.values().map(|v| process_value(v, double_red)).sum();
            if double_red && obj.values().any(|v| v == "red") {
                0
            } else {
                sum
            }
        }
        _ => 0,
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).expect("Error: can't read input file.");

    println!("--- Part 1 ---");
    println!("sum of all numbers: {}", balance_book(&input)); // 111754

    println!("--- Part 2 ---");
    println!("sum of all numbers: {}", balance_book_with_double_red(&input)); // 65402
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prompt_example_1() {
        assert_eq!(balance_book("[1,2,3]"), 6);
        assert_eq!(balance_book("{\"a\":2,\"b\":4}"), 6);
    }
    #[test]
    fn prompt_example_2() {
        assert_eq!(balance_book("[[[3]]]"), 3);
        assert_eq!(balance_book("{\"a\":{\"b\":4},\"c\":-1}"), 3);
    }
    #[test]
    fn prompt_example_3() {
        assert_eq!(balance_book("{\"a\":[-1,1]}"), 0);
        assert_eq!(balance_book("[-1,{\"a\":1}]"), 0);
    }
    #[test]
    fn prompt_example_4() {
        assert_eq!(balance_book("[]"), 0);
        assert_eq!(balance_book("{}"), 0);
    }
    #[test]
    fn part_2_example_1() {
        assert_eq!(balance_book_with_double_red("[1,2,3]"), 6);
    }
    #[test]
    fn part_2_example_2() {
        assert_eq!(
            balance_book_with_double_red("[1,{\"c\":\"red\",\"b\":2},3]"),
            4
        );
    }
    #[test]
    fn part_2_example_3() {
        assert_eq!(
            balance_book_with_double_red("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"),
            0
        );
    }
    #[test]
    fn part_2_example_4() {
        assert_eq!(balance_book_with_double_red("[1,\"red\",5]"), 6);
    }
}
