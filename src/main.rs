mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

fn main() {
    let result = day_06::load_and_evaluate("day_06.txt");
    println!("{} {}", result.0, result.1);
}
