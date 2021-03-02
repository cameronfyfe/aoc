use std::{env, fmt, fs};
use std::convert::TryInto;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
struct Password([char; 8]);

impl Password {
    fn from_str(str: &str) -> Option<Password> {
        match str.chars().collect::<Vec<char>>().try_into() {
            Ok(arr) => Some(Password(arr)),
            _ => None,
        }
    }
    fn is_valid(&self) -> bool {
        // Passwords must include one increasing straight of at least three letters
        self.0
            .iter()
            .tuple_windows()
            .any(|(&r0, &r1, &r2)| {
                let (r1, r2, r3) = (r0 as u32, r1 as u32 - 1, r2 as u32 - 2);
                r1 == r2 && r2 == r3
            })
        // Passwords may not contain the letters i, o, or l
        && self.0
            .iter()
            .any(|&c|
                "iol".contains(c)
            ) == false
        // Passwords must contain at least two different, non-overlapping pairs of letters
        && self.0
            .iter()
            .map(|c| (false, c))
            .coalesce(|a, b|
                if a.0 == false && a.1 == b.1 {
                    Ok((true, a.1))
                } else {
                    Err((a, b))
                }
            )
            .filter(|a| a.0 == true)
            .count() >= 2
    }
    fn next_password(&self) -> Password {
        let mut next_p = self.clone();
        for c in next_p.0.iter_mut().rev() {
            if *c == 'z' {
                *c = 'a';
            } else {
                *c = std::char::from_u32(*c as u32 + 1).unwrap_or(*c);
                break;
            }
        }
        next_p
    }
    fn next_valid_password(&self) -> Password {
        let mut next_p = self.next_password();
        while next_p.is_valid() == false {
            next_p = next_p.next_password();
        }
        next_p
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.iter().collect::<String>())
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = fs::read_to_string(&args[1]).expect("Error: can't read input file.");

    println!("--- Part 1 ---");
    let part_1 = Password::from_str(&input).expect("Invalid starting password").next_valid_password();
    println!("next password: {}", part_1); // cqjxxyzz

    println!("--- Part 2 ---");
    let part_2 = part_1.next_valid_password();
    println!("next password: {}", part_2); // cqkaabcc
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prompt_example_1() {
        assert_eq!(Password::from_str("hijklmmn").unwrap().is_valid(), false);
    }
    #[test]
    fn prompt_example_2() {
        assert_eq!(Password::from_str("abbceffg").unwrap().is_valid(), false);
    }
    #[test]
    fn prompt_example_3() {
        assert_eq!(Password::from_str("abbcegjk").unwrap().is_valid(), false);
    }
    #[test]
    fn prompt_example_4() {
        assert_eq!(
            Password::from_str("abcdefgh")
                .unwrap()
                .next_valid_password(),
            Password::from_str("abcdffaa").unwrap()
        );
    }
    #[test]
    fn prompt_example_5() {
        assert_eq!(
            Password::from_str("ghijklmn")
                .unwrap()
                .next_valid_password(),
            Password::from_str("ghjaabcc").unwrap()
        );
    }
}
