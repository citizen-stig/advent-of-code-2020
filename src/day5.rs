use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() -> (usize, usize) {
    let filename = "input/day5.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let seats: Vec<usize> = reader
        .lines()
        .map(|line| find_seat(&line.unwrap()))
        .map(|(row, column)| row as usize * 8 + column as usize)
        .collect();

    let mut min: Option<usize> = None;
    let mut max: Option<usize> = None;
    let mut sum: usize = 0;
    for seat_id in seats {
        sum += seat_id;
        if min.is_none() || min.gt(&Some(seat_id)) {
            min = Some(seat_id);
        }
        if max.is_none() || max.lt(&Some(seat_id)) {
            max = Some(seat_id);
        }
    }

    let min = min.unwrap();
    let max = max.unwrap();

    let sum_without_gap: usize = (min..max + 1).sum();
    let missed_id = sum_without_gap - sum;
    (max, missed_id)
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
