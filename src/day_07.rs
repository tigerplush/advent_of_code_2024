use std::{fs, result};

pub fn load_and_evaluate(path: &str) -> (i64, i64) {
    let input = fs::read_to_string(path).unwrap();
    (
        evaluate_part_one(input.as_str()),
        evaluate_part_two(input.as_str()),
    )
}

fn evaluate_part_one(input: &str) -> i64 {
    let lines = parse(input);

    let mut correct = Vec::new();
    for line in &lines[0..] {
        let mut current = line.clone();
        let result = current.remove(0);
        let results = solve_part_one(current);
        if results.concat().contains(&result) {
            correct.push(result);
        }
    }
    correct.iter().sum()
}

fn solve_part_one(input: Vec<i64>) -> Vec<Vec<i64>> {
    if input.len() < 2 {
        return vec![input];
    }
    let mut input = input.clone();
    let first = input.remove(0);
    let mut add = input.clone();
    add[0] += first;
    let mut mul = input.clone();
    mul[0] *= first;
    let mut result = solve_part_one(add);
    result.append(&mut solve_part_one(mul));
    result
}

fn solve_part_two(input: Vec<i64>) -> Vec<Vec<i64>> {
    if input.len() < 2 {
        return vec![input];
    }
    let mut input = input.clone();
    let first = input.remove(0);
    let mut add = input.clone();
    add[0] += first;
    let mut mul = input.clone();
    mul[0] *= first;
    let mut concat = input.clone();
    let concat_number = format!("{}{}", first, concat[0]).parse::<i64>().unwrap();
    concat[0] = concat_number;
    let mut result = solve_part_two(add);
    result.append(&mut solve_part_two(mul));
    result.append(&mut solve_part_two(concat));
    result
}

fn evaluate_part_two(input: &str) -> i64 {
    let lines = parse(input);

    let mut correct = Vec::new();
    for line in &lines[0..] {
        let mut current = line.clone();
        let result = current.remove(0);
        let results = solve_part_two(current);
        if results.concat().contains(&result) {
            correct.push(result);
        }
    }
    correct.iter().sum()
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(&[':', ' '])
                .filter(|element| !element.is_empty())
                .map(|element| element.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

const TEST_INPUT_ONE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

#[test]
fn test_part_one() {
    assert_eq!(evaluate_part_one(TEST_INPUT_ONE), 3749);
}
#[test]
fn test_part_two() {
    assert_eq!(evaluate_part_two(TEST_INPUT_ONE), 11387);
}

#[test]
fn test_parse() {
    assert_eq!(parse(TEST_INPUT_ONE)[0], vec![190, 10, 19]);
}
