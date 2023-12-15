use nom;
use std::fs;
use std::num::Wrapping;

const FILE_PATH: &str = "15.txt";

struct Step {
    label: String,
    operation: Option<u8>,
}

impl Step {
    fn serialize(&self) -> String {
        match self.operation {
            None => format!("{}-", self.label),
            Some(value) => format!("{}={}", self.label, value),
        }
    }

    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, (label, operation)) = nom::sequence::tuple((
            nom::character::complete::alpha1,
            nom::branch::alt((parse_minus, parse_equal)),
        ))(input)?;
        Ok((input, Step { label: String::from(label), operation }))
    }
}

fn parse_minus(input: &str) -> nom::IResult<&str, Option<u8>> {
    let (input, _) = nom::bytes::complete::tag("-")(input)?;
    Ok((input, None))
}

fn parse_equal(input: &str) -> nom::IResult<&str, Option<u8>> {
    let (input, value) = nom::sequence::preceded(
        nom::bytes::complete::tag("="), nom::character::complete::u8
    )(input)?;
    Ok((input, Some(value)))
}

fn parse_steps(input: &str) -> nom::IResult<&str, Vec<Step>> {
    nom::multi::separated_list1(
        nom::bytes::complete::tag(","),
        Step::parse,
    )(input)
}

fn hash(value: &str) -> u8 {
    let mut result: Wrapping<u8> = Wrapping(0);
    for byte in value.bytes() {
        result += byte;
        result *= 17;
    }
    result.0
}

fn main() {
    let input = fs::read_to_string(FILE_PATH).unwrap();
    let (_, steps) = parse_steps(&input).unwrap();
    print_full_hashes(&steps);
    print_focusing_power(&steps);
}

fn print_full_hashes(steps: &[Step]) {
    let mut result = 0;
    for step in steps {
        result += hash(&step.serialize()) as usize
    }
    println!("{}", result);
}

fn print_focusing_power(steps: &[Step]) {
    let mut boxes: Vec<Vec<(&str, u8)>> = Vec::new();
    for _ in 0..=u8::MAX {
        boxes.push(Vec::new());
    }
    for step in steps {
        let box_index = hash(&step.label);
        let box_contents = &mut boxes[box_index as usize];
        let position = box_contents.iter().position(|&(label, _)| label == step.label);
        match (step.operation, position) {
            (None, None) => (),
            (None, Some(position)) => _ = box_contents.remove(position),
            (Some(focal_length), None) => box_contents.push((&step.label, focal_length)),
            (Some(focal_length), Some(position)) => box_contents[position] = (&step.label, focal_length),
        }
    }
    let mut result = 0;
    for (box_number, box_contents) in boxes.iter().enumerate() {
        for (slot_number, &(_, focal_length)) in box_contents.iter().enumerate() {
            result += (box_number + 1) * (slot_number + 1) * (focal_length as usize);
        }
    }
    println!("{}", result);
}