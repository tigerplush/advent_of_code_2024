use std::fs;

pub const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

pub fn load_and_evaluate(path: &str) -> (i32, i32) {
    let input = fs::read_to_string(path).unwrap();
    (evaluate(input.as_str()), evaluate_part_two(input.as_str()))
}

fn evaluate(input: &str) -> i32 {
    let (mut lhs, mut rhs) = parse(input);
    lhs.sort();
    rhs.sort();
    
    let mut diffs = Vec::new();
    for i in 0..lhs.len() {
        let diff = (lhs[i] - rhs[i]).abs();
        diffs.push(diff);
    }
    
    diffs.iter().sum()
}

fn evaluate_part_two(input: &str) -> i32 {
    let (lhs, rhs) = parse(input);
    let mut similiarity_score = 0;
    for num in lhs {
        let times = rhs.iter().filter(|e| **e == num).count();
        similiarity_score += num * times as i32;
    }
    similiarity_score
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lhs = input.lines().map(|l| l.split("   ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap()).collect();
    let rhs = input.lines().map(|l| l.split("   ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()).collect();
    (lhs, rhs)
}

#[test]
fn test_evaluate() {
    assert_eq!(evaluate(TEST_INPUT), 11);
}

#[test]
fn test_parse() {
    let (lhs, rhs) = parse(TEST_INPUT);
    assert_eq!(lhs, vec![3, 4, 2, 1, 3, 3]);
    assert_eq!(rhs, vec![4, 3, 5, 3, 9, 3]);
}

#[test]
fn test_evaluate_part_two() {
    assert_eq!(evaluate_part_two(TEST_INPUT), 31);
}