use std::{collections::HashMap, hash::Hash};

use advent_of_code_2024::{Direction, Position};

pub fn load_and_evaluate(input: &str) -> (isize, i32) {
    (evaluate_part_one(input), evaluate_part_two(input))
}

fn evaluate_part_one(input: &str) -> isize {
    let (mut current_pos, mut map, instructions) = parse(input);

    for instruction in instructions.chars() {
        let direction = match instruction {
            '<' => Direction::LEFT,
            '^' => Direction::UP,
            '>' => Direction::RIGHT,
            'v' => Direction::DOWN,
            _ => panic!("unknown instruction"),
        };
        let next = current_pos + direction;
        let pos = map.get(&next);
        match pos {
            Some(object) => {
                match object {
                    Object::Wall => {
                        // do nothing
                    }
                    Object::Box => {
                        try_move_box(&mut map, next, direction);
                        match map.get(&next) {
                            None => {
                                current_pos = next;
                            }
                            _ => ()
                        }
                    }
                }
            }
            None => {
                current_pos = next;
            }
        }
    }

    let mut gps = 0;
    for (pos, obj) in map {
        match obj {
            Object::Box => {
                gps += pos.x + pos.y * 100;
            },
            _ => ()
        }
    }
    gps
}

fn to_map(map: &HashMap<Position, Object>, pos: Position) {
    for y in 0..8 {
        let mut line = String::new();
        for x in 0..8 {
            let c = match map.get(&Position::new(x, y, 0, 0)) {
                Some(obj) => {
                    match obj {
                        Object::Box => 'O',
                        Object::Wall => '#',
                    }
                }
                None => if pos == Position::new(x, y, 0, 0) {
                    '@'
                }
                else {
                    '.'
                }
            };
            line.push(c);
        }
        println!("{}", line);
    }
}

fn try_move_box(map: &mut HashMap<Position, Object>, pos: Position, direction: Direction) {
    let next = pos + direction;
    match map.get(&next) {
        Some(object) => {
            match object {
                Object::Box => {
                    try_move_box(map, next, direction);
                    match map.get(&next) {
                        None => {
                            let box_obj = map.remove(&pos).unwrap();
                            map.insert(next, box_obj);
                        }
                        _ => ()
                    }
                }
                Object::Wall => {}
            }
        }
        None => {
            let box_obj = map.remove(&pos).unwrap();
            map.insert(next, box_obj);
        }
    }
}

fn evaluate_part_two(input: &str) -> i32 {
    0
}


const TEST_INPUT_ONE: &str ="########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

fn parse(input: &str) -> (Position, HashMap<Position, Object>, String){
    let mut lines = input.lines().peekable();
    let mut blocks = vec![];
    while lines.peek().is_some() {
        let block: Vec<_> = lines.by_ref().take_while(|l| l.trim().len() > 0).collect();
        blocks.push(block);
    }

    let map_string = &blocks[0];
    let mut map = HashMap::new();
    let mut starting_position = Position::new(0, 0, 0, 0);
    for y in 0..map_string.len() {
        let line = map_string[y];
        for x in 0..line.len() {
            match line.chars().nth(x).unwrap() {
                '#' => {
                    map.insert(Position::new(x as isize, y as isize, 0, 0), Object::Wall);
                }
                '@' => {
                    starting_position.x = x as isize;
                    starting_position.y = y as isize;
                }
                'O' => {
                    map.insert(Position::new(x as isize, y as isize, 0, 0), Object::Box);
                }
                _ => {

                }
            }
        }
    }

    let instructions = blocks[1].clone();
    let instructions = instructions.join("");
    (starting_position, map, instructions)
}

enum Object {
    Wall,
    Box,
}

const TEST_INPUT_TWO: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

#[test]
fn test_parse() {
    let (pos, map, instructions) = parse(TEST_INPUT_ONE);
    assert_eq!(pos, Position::new(2, 2, 0, 0));
    assert_eq!(instructions, "<^^>>>vv<v>>v<<");
}

#[test]
fn test_part_one() {
    assert_eq!(evaluate_part_one(TEST_INPUT_ONE), 2028);
    assert_eq!(evaluate_part_one(TEST_INPUT_TWO), 10092);
}
#[test]
fn test_part_two() {
    assert_eq!(evaluate_part_two(TEST_INPUT_ONE), 0);
}
