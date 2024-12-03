use std::fs;



const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST_INPUT_TWO: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

pub fn load_and_evaluate(path: &str) -> (i32, i32) {
    let input = fs::read_to_string(path).unwrap();
    (evaluate_part_one(input.as_str()), evaluate_part_two(input.as_str()))
}

fn evaluate_part_one(input: &str) -> i32 {
    let split = input.split(")").collect::<Vec<&str>>();
    let mut parameter_strings = Vec::new();
    for operation in split {
        if let Some(index) = operation.rfind("mul(") {
            let parameter = operation.split_at(index + 4);
            parameter_strings.push(parameter.1);
        }
    }

    let mut multiplied = Vec::new();
    for parameter_string in parameter_strings {
        let split = parameter_string.split(",").collect::<Vec<&str>>();
        if split.len() != 2 {
            continue;
        }
        let Ok(lhs) = split[0].parse::<i32>() else {
            continue;
        };
        let Ok(rhs) = split[1].parse::<i32>() else {
            continue;
        };
        multiplied.push(lhs * rhs);
    }
    multiplied.iter().sum()
}

fn evaluate_part_two(input: &str) -> i32 {
    let split = input.split(")").collect::<Vec<&str>>();
    let mut parameter_strings = Vec::new();
    for operation in split {
        if let Some(index) = operation.rfind("mul(") {
            let parameter = operation.split_at(index + 4);
            parameter_strings.push(parameter.1);
        }
        if let Some(_) = operation.rfind("don't("){
            parameter_strings.push("don't()");
        }
        if let Some(_) = operation.rfind("do("){
            parameter_strings.push("do()");
        }
    }

    let mut enabled = true;
    let mut multiplied = Vec::new();
    println!("{}", parameter_strings.iter().filter(|s| **s == "don't()").count());
    println!("{}", parameter_strings.iter().filter(|s| **s == "do()").count());
    for parameter_string in parameter_strings {
        let split = parameter_string.split(",").collect::<Vec<&str>>();
        // println!("string is {}, multiplying enabled {}", parameter_string, enabled);
        if parameter_string == "don't()" {
            enabled = false;
        } else if parameter_string == "do()" {
            enabled = true;
        }
        if !enabled || split.len() != 2 {
            continue;
        }
        let Ok(lhs) = split[0].parse::<i32>() else {
            continue;
        };
        let Ok(rhs) = split[1].parse::<i32>() else {
            continue;
        };
        multiplied.push(lhs * rhs);
    }
    multiplied.iter().sum()
}

#[test]
fn test_evaluate_part_one() {
    assert_eq!(evaluate_part_one(TEST_INPUT), 161);
}

#[test]
fn test_evaluate_part_two() {
    assert_eq!(evaluate_part_two(TEST_INPUT_TWO), 48);
}