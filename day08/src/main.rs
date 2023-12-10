use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace1, newline, one_of},
    multi::{many0, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use std::cmp;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, direction) = one_of("LR")(input)?;
        Ok((
            input,
            match direction {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!(),
            },
        ))
    }
}

fn parse(input: &str) -> IResult<&str, (Vec<Direction>, Vec<(&str, (&str, &str))>)> {
    let (input, directions) = many0(Direction::parse)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, positions) = separated_list1(
        newline,
        separated_pair(
            alpha1,
            tag(" = "),
            separated_pair(
                preceded(tag("("), alpha1),
                tag(", "),
                terminated(alpha1, tag(")")),
            ),
        ),
    )(input)?;
    Ok((input, (directions, positions)))
}

fn main() {
    let contents = fs::read_to_string("08.txt").unwrap();
    let (_remaining, (directions, parsed_positions)) = parse(&contents).unwrap();
    let mut positions: HashMap<&str, (&str, &str)> = HashMap::new();
    for (pos, (left, right)) in parsed_positions {
        positions.insert(pos, (left, right));
    }
    run(&directions, &positions, |s| s == "AAA", |s| s == "ZZZ");
    run(
        &directions,
        &positions,
        |s| s.ends_with("A"),
        |s| s.ends_with("Z"),
    );
}

fn run(
    directions: &Vec<Direction>,
    positions: &HashMap<&str, (&str, &str)>,
    is_initial_position: fn(&str) -> bool,
    is_final_position: fn(&str) -> bool,
) {
    let mut cycle_intervals: Vec<u64> = Vec::new();
    for (&initial_position, _) in positions {
        if !is_initial_position(initial_position) {
            continue;
        }
        let mut counter = 0;
        let mut old_position = initial_position;
        for direction in directions.iter().cycle() {
            let &(go_left, go_right) = positions.get(old_position).unwrap();
            let new_position = match direction {
                Direction::Left => go_left,
                Direction::Right => go_right,
            };
            old_position = new_position;
            counter += 1;
            if is_final_position(old_position) {
                cycle_intervals.push(counter);
                break;
            }
        }
    }
    println!("{}", least_common_multiple(&cycle_intervals));
}

fn least_common_multiple(values: &[u64]) -> u64 {
    let mut factors: HashMap<u64, u64> = HashMap::new();
    for &value in values {
        let mut factor = 2;
        let mut value = value;
        while factor * factor <= value {
            let mut count = 0;
            while value % factor == 0 {
                value /= factor;
                count += 1;
            }
            if count > 0 {
                let entry = factors.entry(factor).or_insert(0);
                *entry = cmp::max(*entry, count);
            }
            factor += 1;
        }
        if value > 1 {
            factors.entry(value).or_insert(1);
        }
    }
    let mut result: u64 = 1;
    for (factor, count) in factors {
        for _ in 0..count {
            result *= factor;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_common_multiple() {
        assert_eq!(least_common_multiple(&[1, 4, 7]), 28);
        assert_eq!(least_common_multiple(&[9, 36]), 36);
    }
}