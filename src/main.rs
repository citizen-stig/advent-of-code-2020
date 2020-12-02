use std::env;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let run_all = args.len() >= 2 && args.get(1).unwrap() == "--all";
    if run_all {
        println!("======= Day 1 ========");
        let day1_solution = day1::solve();
        println!("Solution for day 1: {:?} ", day1_solution);
    }
    println!("======= Day 2 ========");
    let day2_solution = day2::solve();
    println!("Solution for day 1: {:?} ", day2_solution);
}
