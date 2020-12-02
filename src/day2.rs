use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() -> (usize, usize) {
    let filename = "input/day2.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<(String, char, (u32, u32))> = reader
        .lines()
        .map(|line| parse_line(line.unwrap().as_ref()))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    let result1 = lines
        .iter()
        .filter(|input| is_password_valid(&input.0, &input.1, &input.2))
        .count();

    let result2 = lines
        .iter()
        .filter(|input| is_password_valid_part_2(&input.0, &input.1, &input.2))
        .count();

    (result1, result2)
}

fn parse_line(line: &str) -> Option<(String, char, (u32, u32))> {
    let v: Vec<&str> = line.split(' ').collect();
    if v.len() != 3 {
        println!("Incorrect string: '{:?}'", line);
        return None;
    }
    let letter = v.get(1).unwrap();
    if letter.len() != 2 {
        println!("Incorrect char: {:?}", letter);
        return None;
    }
    let letter: char = letter.chars().next().unwrap();
    let range: Vec<&str> = v.get(0).unwrap().split('-').collect();
    if range.len() != 2 {
        println!("Incorrect range: {:?}", range)
    }
    let min = range.get(0).unwrap().parse::<u32>().unwrap();
    let max = range.get(1).unwrap().parse::<u32>().unwrap();

    Some((v.get(2).unwrap().to_string(), letter, (min, max)))
}

fn is_password_valid(password: &str, letter: &char, range: &(u32, u32)) -> bool {
    let letter_count = password.chars().filter(|c| c == letter).count();
    letter_count >= range.0 as usize && letter_count <= range.1 as usize
}

fn is_password_valid_part_2(password: &str, letter: &char, range: &(u32, u32)) -> bool {
    let has_letter_on_position =
        |position: usize| password.chars().nth(position - 1).unwrap() == *letter;
    has_letter_on_position(range.0 as usize) ^ has_letter_on_position(range.1 as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod is_password_valid {
        use super::*;

        #[test]
        fn valid_passwords() {
            assert!(is_password_valid("abcde", &'a', &(1, 3)));
            assert!(is_password_valid("ccccccccc", &'c', &(2, 9)));
        }

        #[test]
        fn invalid_passwords() {
            assert!(!is_password_valid("cdefg", &'b', &(1, 3)));
            assert!(!is_password_valid("cdefg", &'c', &(2, 3)));
            assert!(!is_password_valid("aaccaaa", &'a', &(1, 3)));
        }
    }

    mod is_password_valid_part_2 {
        use super::*;

        #[test]
        fn valid_passwords() {
            assert!(is_password_valid_part_2("abcde", &'a', &(1, 3)));
            assert!(is_password_valid_part_2("aabbddeecq", &'c', &(2, 9)));
        }

        #[test]
        fn invalid_passwords() {
            assert!(!is_password_valid_part_2("icdefg", &'b', &(1, 3)));
            assert!(!is_password_valid_part_2("ccccccccc", &'c', &(2, 9)));
        }
    }

    mod parse_line {
        use super::*;

        #[test]
        fn test_valid() {
            assert_eq!(
                Some(("rrrmrr".to_string(), 'r', (5, 6))),
                parse_line("5-6 r: rrrmrr")
            );
        }
    }
}
