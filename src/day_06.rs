use std::{collections::HashSet, fs, ops::Add};

use advent_of_code_2024::{Position, DIRECTIONS};

pub fn load_and_evaluate(path: &str) -> (usize, i32) {
    let input = fs::read_to_string(path).unwrap();
    (
        evaluate_part_one(input.as_str()),
        evaluate_part_two(input.as_str()),
    )
}


fn evaluate_part_one(input: &str) -> usize {
    let height = input.lines().count();
    let input = input.lines().collect::<Vec<&str>>();
    let width = input[0].len();
    let input = input.concat();
    let start = input.find("^").unwrap();
    let mut current_position = Position::new((start % width) as isize, (start / width) as isize, width, height);
    let mut start_direction = 0;

    let mut positions = Vec::new();
    positions.push(current_position);
    while current_position.in_bounds() {
        let new_position = current_position + DIRECTIONS[start_direction];
        if !new_position.in_bounds() {
            break;
        }
        if input.chars().nth(new_position.index()).unwrap() == '#' {
            start_direction += 1;
            start_direction = start_direction % 4;
        } else {
            current_position = new_position;
            positions.push(new_position);
        }
    }
    let hash = positions.iter().cloned().collect::<HashSet<Position>>();
    hash.len()
}


fn evaluate_part_two(input: &str) -> i32 {
    0
}

const TEST_INPUT_ONE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[test]
fn test_part_one() {
    assert_eq!(evaluate_part_one(TEST_INPUT_ONE), 41);
}
#[test]
fn test_part_two() {
    assert_eq!(evaluate_part_two(TEST_INPUT_ONE), 0);
}
