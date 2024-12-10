use std::fs;

pub fn load_and_evaluate(path: &str) -> (i32, i32) {
    let input = fs::read_to_string(path).unwrap();
    (
        evaluate_part_one(input.as_str()),
        evaluate_part_two(input.as_str()),
    )
}

fn evaluate_part_one(input: &str) -> i32 {
    let chars = parse(input);
    let mut result = 0;
    for y in 0..chars.len() {
        for x in 0..chars[y].len() {
            for direction in Direction::DIRECTIONS {
                if x as isize + direction.x * 3 >= chars[y].len() as isize
                    || x as isize + direction.x * 3 < 0
                {
                    continue;
                }
                if y as isize + direction.y * 3 >= chars[y].len() as isize
                    || y as isize + direction.y * 3 < 0
                {
                    continue;
                }
                let match_x = chars[(y as isize + direction.y * 0) as usize]
                    [(x as isize + direction.x * 0) as usize]
                    == 'X';
                let match_m = chars[(y as isize + direction.y * 1) as usize]
                    [(x as isize + direction.x * 1) as usize]
                    == 'M';
                let match_a = chars[(y as isize + direction.y * 2) as usize]
                    [(x as isize + direction.x * 2) as usize]
                    == 'A';
                let match_s = chars[(y as isize + direction.y * 3) as usize]
                    [(x as isize + direction.x * 3) as usize]
                    == 'S';
                if match_x && match_m && match_a && match_s {
                    result += 1;
                }
            }
        }
    }
    result
}

fn evaluate_part_two(input: &str) -> i32 {
    let chars = parse(input);
    let mut result = 0;

    for y in 1..chars.len() - 1 {
        for x in 1..chars[y].len() - 1 {
            if chars[y][x] != 'A' {
                continue;
            }
            // we have an A
            let mas = chars[y - 1][x - 1] == 'M' && chars[y + 1][x + 1] == 'S';
            let sam = chars[y - 1][x - 1] == 'S' && chars[y + 1][x + 1] == 'M';
            let mas_i = chars[y - 1][x + 1] == 'M' && chars[y + 1][x - 1] == 'S';
            let sam_i = chars[y - 1][x + 1] == 'S' && chars[y + 1][x - 1] == 'M';
            if (mas || sam) && (mas_i || sam_i) {
                result += 1;
            }
        }
    }

    result
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.chars().collect());
    }
    result
}

struct Direction {
    x: isize,
    y: isize,
}

impl Direction {
    const RIGHT: Direction = Direction::new(1, 0);
    const LEFT: Direction = Direction::new(-1, 0);
    const UP: Direction = Direction::new(0, -1);
    const DOWN: Direction = Direction::new(0, 1);
    const DOWN_RIGHT: Direction = Direction::new(1, 1);
    const DOWN_LEFT: Direction = Direction::new(-1, 1);
    const UP_RIGHT: Direction = Direction::new(1, -1);
    const UP_LEFT: Direction = Direction::new(-1, -1);

    const DIRECTIONS: [Direction; 8] = [
        Self::RIGHT,
        Self::LEFT,
        Self::UP,
        Self::DOWN,
        Self::DOWN_RIGHT,
        Self::DOWN_LEFT,
        Self::UP_RIGHT,
        Self::UP_LEFT,
    ];

    const fn new(x: isize, y: isize) -> Self {
        Direction { x, y }
    }
}

const TEST_INPUT_ONE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[test]
fn test_part_one() {
    assert_eq!(evaluate_part_one(TEST_INPUT_ONE), 18);
}
#[test]
fn test_part_two() {
    assert_eq!(evaluate_part_two(TEST_INPUT_ONE), 9);
}

#[test]
fn test_parse() {
    assert_eq!(
        parse(TEST_INPUT_ONE)[0],
        vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M']
    );
}
