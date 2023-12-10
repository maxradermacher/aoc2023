use nom::{character, multi::separated_list1, IResult};
use std::fs;

fn parse(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(character::complete::space1, character::complete::i64)(input)
}

fn main() {
    let input = fs::read_to_string("09.txt").unwrap();
    run(&input, false);
    run(&input, true);
}

fn run(input: &str, reverse: bool) {
    let mut result = 0;
    for line in input.lines() {
        let (_, mut values) = parse(line).unwrap();
        if reverse {
            values.reverse();
        }
        result += extrapolate_forward(&values);
    }
    println!("{}", result);
}

fn extrapolate_forward(values: &Vec<i64>) -> i64 {
    let mut derivatives: Vec<i64> = values.clone();
    for order in 1..derivatives.len() {
        for idx in 0..(derivatives.len() - order) {
            derivatives[idx] = derivatives[idx + 1] - derivatives[idx]
        }
    }
    derivatives.iter().sum()
}
