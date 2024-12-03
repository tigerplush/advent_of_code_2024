use std::fs;

pub fn load_and_evaluate(path: &str) -> (usize, usize) {
    let input = fs::read_to_string(path).unwrap();
    (evaluate(input.as_str()), evaluate_part_two(input.as_str()))
}

const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

fn evaluate(input: &str) -> usize {
    let parsed_input = parse(input);
    let reports = parsed_input.len();
    let mut unsafe_report = 0;
    for report in parsed_input {
        let levels = report
            .iter()
            .enumerate()
            .skip(1)
            .map(|(a, b)| report[a - 1] - *b)
            .collect::<Vec<i32>>();
        if levels.iter().any(|e| e.abs() > 3)
            | (!levels.iter().all(|e| e.signum() > 0) && !levels.iter().all(|e| e.signum() < 0))
        {
            unsafe_report += 1;
        }
    }
    reports - unsafe_report
}

fn evaluate_part_two(input: &str) -> usize {
    let original_reports = parse(input);
    let mut reports = original_reports
        .iter()
        .map(|report| Report::new(report.clone()))
        .collect::<Vec<Report>>();
    let previous = reports.len();
    reports.iter_mut().for_each(|e| e.fix());
    reports.retain(|report| !report.is_valid());
    let diff = previous - reports.len();
    // panic!("I give up");
    diff
}

struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn new(levels: Vec<i32>) -> Self {
        Report { levels }
    }

    fn is_valid(&self) -> bool {
        let derivation = self.levels.derive();
        if derivation.iter().any(|e| e.abs() > 3) | derivation.iter().any(|e| e.abs() == 0) {
            return false;
        }
        if !derivation.iter().all(|e| e.signum() > 0) && !derivation.iter().all(|e| e.signum() < 0)
        {
            return false;
        }
        true
    }

    fn fix(&mut self) {
        // early exit
        if self.is_valid() {
            return;
        }

        println!("checking {:?}...", self.levels);
            for i in 0..self.levels.len()  {
                let mut copy = self.levels.clone();
                copy.remove(i);
                if Report::new(copy).is_valid() {
                    println!("new variant with removal of {} is valid", self.levels[i]);
                    self.levels.remove(i);
                    break;
                }
            }
    }
}

trait Derivation {
    fn derive(&self) -> Vec<i32>;
}

impl Derivation for Vec<i32> {
    fn derive(&self) -> Vec<i32> {
        self.iter()
            .enumerate()
            .skip(1)
            .map(|(a, b)| self[a - 1] - *b)
            .collect::<Vec<i32>>()
    }
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut reports = Vec::new();
    for line in input.lines() {
        let levels = line.split(" ").map(|e| e.parse::<i32>().unwrap()).collect();
        reports.push(levels);
    }
    reports
}

#[test]
fn test_evaluate() {
    assert_eq!(evaluate(TEST_INPUT), 2);
}

#[test]
fn test_parse() {
    let parsed_input = parse(TEST_INPUT);
    assert_eq!(parsed_input[0], vec![7, 6, 4, 2, 1]);
    assert_eq!(parsed_input[1], vec![1, 2, 7, 8, 9]);
    assert_eq!(parsed_input[2], vec![9, 7, 6, 2, 1]);
    assert_eq!(parsed_input[3], vec![1, 3, 2, 4, 5]);
}

#[test]
fn test_validity() {
    let report = Report::new(vec![7, 6, 4, 2, 1]);
    assert!(report.is_valid());
}

#[test]
fn test_evaluate_part_two() {
    //assert_eq!(evaluate_part_two(TEST_INPUT), 4);
}
