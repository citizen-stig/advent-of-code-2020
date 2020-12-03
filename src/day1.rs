use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

pub fn solve() -> (i32, i32) {
    let filename = "input/day1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    let data_set: HashSet<i32> = HashSet::from_iter(data.iter().cloned());

    (
        two_numbers_product(&data_set, 2020).unwrap(),
        three_numbers_product(&data_set, 2020).unwrap(),
    )
}

fn two_numbers(data_set: &HashSet<i32>, target: i32) -> Option<(i32, i32)> {
    for number in data_set {
        let second_part = target - number;
        if data_set.contains(&second_part) {
            return Some((*number, second_part));
        }
    }
    None
}

fn two_numbers_product(data_set: &HashSet<i32>, target: i32) -> Option<i32> {
    match two_numbers(data_set, target) {
        Some((number_one, number_two)) => Some(number_one * number_two),
        None => None,
    }
}

fn three_numbers_product(data_set: &HashSet<i32>, target: i32) -> Option<i32> {
    for number in data_set {
        let second_part = target - number;
        let two_part = two_numbers(&data_set, second_part);
        if let Some(other_two) = two_part {
            return Some(number * other_two.0 * other_two.1);
        }
    }
    None
}
