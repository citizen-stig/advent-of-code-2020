use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() -> (i64, i64) {
    let filename = "input/day3.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let trees: Vec<i64> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(step_right, step_down)| count_trees(&lines, *step_right, *step_down))
        .collect();
    let part_two_answer = trees.iter().product();
    // println!("Trees: {:?}", trees);
    (*trees.get(1).unwrap(), part_two_answer)
}

fn count_trees(lines: &[String], step_right: usize, step_down: usize) -> i64 {
    let mut position: usize = 0;
    lines
        .iter()
        .enumerate()
        .map(|(idx, line)| {
            if idx > 0 && idx % step_down != 0 {
                return 0;
            }
            let c = line.chars().nth(position);
            // println!("LINE: {:?}; [{:?}] = {:?}", line, position, c);
            let result = if c == Some('#') { 1 } else { 0 };
            position = (position + step_right) % line.len();
            result
        })
        .sum()
}
