use std::{cmp::Ordering, fs};


pub fn load_and_evaluate(path: &str) -> (i32, i32) {
    let input = fs::read_to_string(path).unwrap();
    (evaluate_part_one(input.as_str()), evaluate_part_two(input.as_str()))
}

fn evaluate_part_one(input: &str) -> i32 {
    let (rules, pages) = parse(input);

    let mut correct = Vec::new();
    for page in pages {
        if page.evaluate(&rules) {
            correct.push(page);
        }
    }
    
    let mut result = 0;
    for pages in correct {
        let mid = pages.len() / 2;
        result += pages[mid];
    }
    result
}

fn evaluate_part_two(input: &str) -> i32 {
    let (rules, mut pages) = parse(input);
    pages.retain(|p| !p.evaluate(&rules));
    let mut correct = Vec::new();
    for mut page in pages {
        page.sort_by(|lhs, rhs| {
            for rule in &rules {
                if rule.lhs == *rhs && rule.rhs == *lhs {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        });
        correct.push(page);
    }
    
    let mut result = 0;
    for pages in correct {
        let mid = pages.len() / 2;
        result += pages[mid];
    }
    result
}

fn parse(input: &str) -> (Vec<PageOrderingRule>, Vec<Vec<i32>>){
    let rules = input.lines().take_while(|l| !l.is_empty()).collect::<Vec<&str>>();
    let pages = input.lines().skip_while(|l| !l.is_empty()).skip(1).collect::<Vec<&str>>();
    let rules = rules.iter().map(|e| PageOrderingRule::parse_from(*e)).collect::<Vec<PageOrderingRule>>();
    let pages = pages.iter().map(|l| l.split(",").map(|e| e.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    (rules, pages)
}

#[derive(Debug, PartialEq)]
struct PageOrderingRule {
    lhs: i32,
    rhs: i32,
}

impl PageOrderingRule {
    fn parse_from(input: &str) -> Self {
        let nums = input.split("|").map(|f| f.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        Self {
            lhs: nums[0],
            rhs: nums[1],
        }
    }

    fn evaluate(&self, window: &[i32]) -> bool {
        if self.lhs == window[1] && self.rhs == window[0] {
            false
        }
        else {
            true
        }
    }
}

trait Evaluate {
    fn evaluate(&self, rules: &Vec<PageOrderingRule>) -> bool;
}

impl Evaluate for Vec<i32> {
    fn evaluate(&self, rules: &Vec<PageOrderingRule>) -> bool {
        for window in self.windows(2) {
            for rule in rules {
                if rule.evaluate(window) == false {
                    return false;
                }
            }
        }
        true
    }
}

const TEST_INPUT_ONE: &str ="47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

#[test]
fn test_part_one() {
    assert_eq!(evaluate_part_one(TEST_INPUT_ONE), 143);
}
#[test]
fn test_part_two() {
    assert_eq!(evaluate_part_two(TEST_INPUT_ONE), 123);
}

#[test]
fn test_parse() {
    let (rules, pages) = parse(TEST_INPUT_ONE);
    assert_eq!(rules[0], PageOrderingRule::parse_from("47|53"));
    assert_eq!(pages[0], vec![75, 47, 61,53, 29]);
}