use std::collections::HashMap;

pub fn load_and_evaluate(input: &str) -> (usize, usize) {
    (0, evaluate_part_two(input))
}

fn evaluate_part_one(input: &str) -> usize {
    let mut input = input.split(" ").map(|element| element.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    for _ in 0..25 {
        input = blink(&input);
    }
    input.len()
}

fn evaluate_part_two(input: &str) -> usize {
    let input = input.split(" ").map(|element| element.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let mut hash_map = HashMap::new();
    for num in input {
        hash_map.entry(num).and_modify(|e| *e += 1).or_insert(1);
    }
    for _ in 0..75 {
        hash_map = blink_dict(hash_map);
    }
    let mut result = 0;
    for (_stone, amount) in hash_map {
        result += amount;
    }
    result
}

fn blink_dict(input: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut result = HashMap::new();
    for (num, amount) in input {
        if num == 0 {
            result.entry(num + 1).and_modify(|e| *e += amount).or_insert(amount);
        }
        else if format!("{}", num).len() % 2 == 0 {
            let num_string = format!("{}", num);
            let (lhs, rhs) = num_string.split_at(num_string.len() / 2);
            result.entry(lhs.parse::<u64>().unwrap()).and_modify(|e| *e += amount).or_insert(amount);
            result.entry(rhs.parse::<u64>().unwrap()).and_modify(|e| *e += amount).or_insert(amount);
        }
        else {
            result.entry(num * 2024).and_modify(|e| *e += amount).or_insert(amount);
        }
    }
    result
}

fn blink(input: &Vec<u64>) -> Vec<u64> {
    let mut result = Vec::new();
    for num in input {
        if *num == 0 {
            result.push(1);
        }
        else if format!("{}", num).len() % 2 == 0 {
            let num_string = format!("{}", num);
            let (lhs, rhs) = num_string.split_at(num_string.len() / 2);
            result.push(lhs.parse::<u64>().unwrap());
            result.push(rhs.parse::<u64>().unwrap());
        }
        else {
            result.push(num * 2024);
        }
    }
    result
}

const TEST_INPUT_ONE: &str ="0 1 10 99 999";

#[test]
fn test_blink() {
    let input = TEST_INPUT_ONE.split(" ").map(|element| element.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    assert_eq!(blink(&input), vec![1,2024,1,0,9,9,2021976]);
}

#[test]
fn test_part_one() {
    assert_eq!(evaluate_part_one("125 17"), 55312);
}

#[test]
fn test_part_two() {
    assert_eq!(evaluate_part_two("125 17"), 55312);
}
