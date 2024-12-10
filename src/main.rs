mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

fn main() {
    let result = day_07::load_and_evaluate("day_07.txt");
    println!("{} {}", result.0, result.1);
}
