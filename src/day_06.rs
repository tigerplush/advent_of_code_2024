use std::{collections::HashSet, fs, ops::Add};

pub fn load_and_evaluate(path: &str) -> (usize, i32) {
    let input = fs::read_to_string(path).unwrap();
    (
        evaluate_part_one(input.as_str()),
        evaluate_part_two(input.as_str()),
    )
}

const DIRECTIONS: [Position; 4] = [
    Position::new(0, -1),
    Position::new(1, 0),
    Position::new(0, 1),
    Position::new(-1, 0),
];

fn evaluate_part_one(input: &str) -> usize {
    let height = input.lines().count();
    let input = input.lines().collect::<Vec<&str>>();
    let width = input[0].len();
    let input = input.concat();
    let start = input.find("^").unwrap();
    let mut current_position = Position::from((start % width, start / width));
    let mut start_direction = 0;

    println!("{} {}", width, height);

    let mut positions = Vec::new();
    positions.push(current_position);
    while in_bounds(current_position, width as isize, height as isize) {
        let new_position = current_position + DIRECTIONS[start_direction];
        if !in_bounds(new_position, width as isize, height as isize) {
            break;
        }
        let index = new_position.x as usize + new_position.y as usize * height;
        if input.chars().nth(index).unwrap() == '#' {
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

fn in_bounds(position: Position, width: isize, height: isize) -> bool {
    position.x >= 0 && position.x < width && position.y >= 0 && position.y < height
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    const fn new(x: i32, y: i32) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

impl From<(i32, i32)> for Position {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
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
