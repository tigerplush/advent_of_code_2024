use std::{collections::HashMap, fs};

use advent_of_code_2024::{Direction, Position, DIRECTIONS};


pub fn load_and_evaluate(input: &str) -> (usize, i32) {
    (evaluate_part_one(input), evaluate_part_two(input))
}

fn evaluate_part_one(input: &str) -> usize {
    let height = input.lines().count();
    let input = input.lines().collect::<Vec<&str>>();
    let width = input[0].len();
    let input = input.concat();

    let mut current_position = Position::new(0, 0, width, height);
    let mut regions: Vec<(char, Vec<Position>)> = Vec::new();
    let mut visited = Vec::new();

    while current_position.in_bounds() {
        if !visited.contains(&current_position) {
            let current_region = input.chars().nth(current_position.index()).unwrap();
            let mut region = Vec::new();
            let mut frontier = vec![current_position];
            while let Some(current) = frontier.pop() {
                region.push(current);
                visited.push(current);
                for direction in DIRECTIONS {
                    let next = current + direction;
                    if !next.in_bounds() || region.contains(&next){
                        continue;
                    }
                    let next_region = input.chars().nth(next.index()).unwrap();
                    if current_region == next_region && !frontier.contains(&next){
                        frontier.push(next);
                    }
                }
            }
            regions.push((current_region, region));
        }
        current_position.advance();
    }

    let mut price = 0;
    for (label, plots) in regions {
        let region = Region {
            label,
            plots
        };
        price += region.price();
    }
    price
}

fn evaluate_part_two(input: &str) -> i32 {
    0
}

const TEST_INPUT_ONE: &str = "AAAA
BBCD
BBCC
EEEC";

const TEST_INPUT_TWO: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";


const TEST_INPUT_THREE: &str ="RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

#[test]
fn test_part_one() {
    assert_eq!(evaluate_part_one(TEST_INPUT_ONE), 140);
    assert_eq!(evaluate_part_one(TEST_INPUT_TWO), 772);
    assert_eq!(evaluate_part_one(TEST_INPUT_THREE), 1930);
}
#[test]
fn test_part_two() {
    assert_eq!(evaluate_part_two(TEST_INPUT_ONE), 0);
}

struct Region {
    label: char,
    plots: Vec<Position>,
}

impl Region {
    fn price(&self) -> usize {
        let mut fence = Vec::new();
        for position in &self.plots {
            for direction in DIRECTIONS {
                let next = *position + direction;
                if !self.plots.contains(&next) {
                    fence.push(next);
                }
            }
        }
        self.plots.len() * fence.len()
    }

    fn side_price(&self) -> usize {
        let mut fence = Vec::new();
        for position in &self.plots {
            for direction in DIRECTIONS {
                let next = *position + direction;
                if !self.plots.contains(&next) {
                    fence.push(next);
                }
            }
        }
        self.plots.len() * fence.len()
    }
}
