use std::fs;


pub fn load_and_evaluate(input: &str) -> (i32, i32) {
    (evaluate_part_one(input), evaluate_part_two(input))
}

fn evaluate_part_one(input: &str) -> i32 {
    0
}

fn evaluate_part_two(input: &str) -> i32 {
    0
}


const TEST_INPUT_ONE: &str ="............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

#[test]
fn test_part_one() {
    assert_eq!(evaluate_part_one(TEST_INPUT_ONE), 0);
}
#[test]
fn test_part_two() {
    assert_eq!(evaluate_part_two(TEST_INPUT_ONE), 0);
}
