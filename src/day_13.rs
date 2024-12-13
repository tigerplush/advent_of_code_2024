use advent_of_code_2024::{Direction, Position};


pub fn load_and_evaluate(input: &str) -> (isize, isize) {
    (evaluate_part_one(input), evaluate_part_two(input))
}

fn evaluate_part_one(input: &str) -> isize {
    let machines = parse(input);

    let mut tokens = 0;
    for (a, b, price) in machines {
        let nom = price.x * b.y - price.y * b.x;
        let denom = a.x * b.y - a.y * b.x;
        let u1 = nom as f32 / denom as f32;
        if u1.fract() != 0.0 {
            continue;
        }
        let u1 = u1 as isize;
        let u2 = (price.y - u1 * a.y) / b.y;
        if u1 >= 0 && u1 <= 100 && u2 >= 0 && u2 <= 100 {
            tokens += u1 * 3 + u2
        }
    }
    tokens
}
fn evaluate_part_two(input: &str) -> isize {
    let machines = parse(input);

    let mut tokens = 0;
    for (a, b, mut price) in machines {
        price.x += 10000000000000;
        price.y += 10000000000000;
        let nom = price.x * b.y - price.y * b.x;
        let denom = a.x * b.y - a.y * b.x;
        if nom % denom != 0 {
            continue;
        }
        let u1 = nom / denom;
        let u2 = (price.y - u1 * a.y) / b.y;
        println!("{:?} {:?} {:?}: {} {}", a, b, price, u1, u2);
        if u1 >= 0 && u2 >= 0 {
            tokens += u1 * 3 + u2
        }
    }
    tokens
}


const TEST_INPUT_ONE: &str ="Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

fn parse(input: &str) -> Vec<(Direction, Direction, Position)>{
    let mut lines = input.lines().peekable();
    let mut blocks = vec![];
    while lines.peek().is_some() {
        let block: Vec<_> = lines.by_ref().take_while(|l| l.trim().len() > 0).collect();
        blocks.push(block);
    }
    
    let mut machines = Vec::new();
    for block in blocks {
        let index = block[0].find("X+").unwrap();
        let a = block[0].split_at(index).1.split(", ").collect::<Vec<&str>>();
        let x = a[0][2..].parse::<isize>().unwrap();
        let y = a[1][2..].parse::<isize>().unwrap();
        let a = Direction::new(x, y);

        let index = block[1].find("X+").unwrap();
        let b = block[1].split_at(index).1.split(", ").collect::<Vec<&str>>();
        let x = b[0][2..].parse::<isize>().unwrap();
        let y = b[1][2..].parse::<isize>().unwrap();
        let b = Direction::new(x, y);

        let index = block[2].find("X=").unwrap();
        let p = block[2].split_at(index).1.split(", ").collect::<Vec<&str>>();
        let x = p[0][2..].parse::<isize>().unwrap();
        let y = p[1][2..].parse::<isize>().unwrap();
        let p = Position::new(x, y, 0, 0);

        machines.push((a, b, p));
    }
    machines
}

#[test]
fn test_parse() {
    let machines = parse(TEST_INPUT_ONE);
    assert_eq!(machines[0], (Direction::new(94, 34), Direction::new(22, 67), Position::new(8400, 5400, 0, 0)));
}

#[test]
fn test_part_one() {
    assert_eq!(evaluate_part_one(TEST_INPUT_ONE), 480);
}
#[test]
fn test_part_two() {
    assert_eq!(evaluate_part_two(TEST_INPUT_ONE), 0);
}
