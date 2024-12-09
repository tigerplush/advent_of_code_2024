mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

fn main() {
    let result = day_05::load_and_evaluate("day_05.txt");
    println!("{} {}", result.0, result.1);
}
