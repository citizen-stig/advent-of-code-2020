use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() -> u16 {
    let filename = "input/day5.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| find_seat(&line.unwrap()))
        .map(|(row, column)| row as u16 * 8 + column as u16)
        .max()
        .unwrap()
}

fn binary_partition(c: &char, min: u8, max: u8) -> (u8, u8) {
    let diff = max - min;
    let half = diff / 2;
    match c {
        'F' | 'L' => (min, max - half - 1),
        'B' | 'R' => (max - half, max),
        _ => panic!("Unexpected symbol: {:?}", c),
    }
}

fn find_from_input(input: &str, mut min: u8, mut max: u8) -> u8 {
    for c in input.chars() {
        // (min, max) = binary_partition(&c, min, max);
        let result = binary_partition(&c, min, max);
        min = result.0;
        max = result.1;
    }
    min
}

fn find_seat(input: &str) -> (u8, u8) {
    let row = find_from_input(&input[0..7], 0, 127);
    let column = find_from_input(&input[7..10], 0, 7);
    (row, column)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fb_partition() {
        assert_eq!((0, 63), binary_partition(&'F', 0, 127));
        assert_eq!((64, 127), binary_partition(&'B', 0, 127));
        assert_eq!((0, 31), binary_partition(&'F', 0, 63));
        assert_eq!((32, 63), binary_partition(&'B', 0, 63));
        assert_eq!((32, 47), binary_partition(&'F', 32, 63));
        assert_eq!((48, 63), binary_partition(&'B', 32, 63));
        assert_eq!((40, 47), binary_partition(&'B', 32, 47));
        assert_eq!((32, 39), binary_partition(&'F', 32, 47));
        assert_eq!((44, 47), binary_partition(&'B', 40, 47));
        assert_eq!((44, 45), binary_partition(&'F', 44, 47));
        assert_eq!((44, 44), binary_partition(&'F', 44, 45));
    }

    #[test]
    fn test_find_row() {
        assert_eq!(44, find_from_input("FBFBBFF", 0, 127));
    }

    #[test]
    fn test_find_column() {
        assert_eq!(5, find_from_input("RLR", 0, 7));
    }

    #[test]
    fn test_find_seat() {
        assert_eq!((44, 5), find_seat("FBFBBFFRLR"));
    }
}
