use std::env;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let run_all = args.len() >= 2 && args.get(1).unwrap() == "--all";
    if run_all {
        println!("======= Day 1 ========");
        let day1_solution = day1::solve();
        println!("Solution for day 1: {:?} ", day1_solution);
        println!("======= Day 2 ========");
        let day2_solution = day2::solve();
        println!("Solution for day 2: {:?} ", day2_solution);
        println!("======= Day 3 ========");
        let day3_solution = day3::solve();
        println!("Solution for day 3: {:?} ", day3_solution);
    }
    println!("======= Day 4 ========");
    let day4_solution = day4::solve();
    println!("Solution for day 4: {:?} ", day4_solution);
}
