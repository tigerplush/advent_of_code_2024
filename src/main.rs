use std::fs;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_11;
mod day_12;

fn main() {
    let input = fs::read_to_string("day_11.txt").unwrap();
    let result = day_11::load_and_evaluate(&input);
    println!("{} {}", result.0, result.1);
}
