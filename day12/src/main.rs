use nom;
use std::fs;

const FILE_PATH: &str = "12.txt";

#[derive(Clone, Copy, Debug)]
enum State {
    Operational,
    Damaged,
}

#[derive(Debug)]
struct Row {
    states: Vec<Option<State>>,
    groups: Vec<u8>,
}

impl Row {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, (states, groups)) = nom::sequence::separated_pair(
            nom::multi::many1(nom::character::complete::one_of("#.?")),
            nom::character::complete::space1,
            nom::multi::separated_list1(
                nom::bytes::complete::tag(","),
                nom::character::complete::u8
            )
        )(input)?;
        Ok((input, Row{
            states: states.into_iter().map(|v| match v {
                '.' => Some(State::Operational),
                '#' => Some(State::Damaged),
                '?' => None,
                _ => unreachable!(),
            }).collect(),
            groups
        }))
    }
}

// .??..??...?##.?.??..??...?##.?.??..??...?##.

fn main() {
    let mut result = 0;
    let input = fs::read_to_string(FILE_PATH).unwrap();
    for line in input.lines() {
        let (_, row) = Row::parse(line).unwrap();
        let mut states = row.states;
        let groups = row.groups;
        result += count_placements(&mut states, &groups);
    }
    println!("{}", result);
}

fn count_placements(states: &[Option<State>], groups: &[u8]) -> u32 {
    match groups.first() {
        None => {
            let can_place = states.iter().all(|s| matches!(s, None | Some(State::Operational)));
            if can_place { 1 } else { 0 }
        }
        Some(&size) => {
            let mut result = 0;
            let size: usize = size.try_into().unwrap();
            for (lower, state) in states.iter().enumerate() {
                if matches!(state, Some(State::Operational)) {
                    continue;
                }
                if matches!(state, None | Some(State::Damaged)) {
                    result += count_placement(&states[lower..], size, &groups[1..]);
                }
                if matches!(state, Some(State::Damaged)) {
                    break;
                }
            }
            result
        }
    }
}

fn count_placement(mut states: &[Option<State>], size: usize, groups: &[u8]) -> u32 {
    // If we can't place `size` Damaged states, we can't place anything here.
    if states.len() < size {
        return 0;
    }
    if !states.iter().take(size).all(|s| matches!(s, None | Some(State::Damaged))) {
        return 0;
    }
    // If we can't follow it with an Operational state (or the end), we can't place anything here.
    states = &states[size..];
    match states.first() {
        None => (),
        Some(None | Some(State::Operational)) => states = &states[1..],
        Some(Some(State::Damaged)) => {
            return 0
        },
    };
    return count_placements(states, groups);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extra_group() {
        assert_eq!(count_placements(&[], &[3]), 0);
    }

    #[test]
    fn test_unknown_damaged_unknown() {
        assert_eq!(
            count_placements(&[None, Some(State::Damaged), None], &[1]),
            1
        );
    }

    #[test]
    fn test_multiple_unknown_then_operational() {
        assert_eq!(
            count_placements(&[None, None, None, None, Some(State::Operational)], &[1]),
            4
        );
    }
}
