use nom::{
    branch::alt, bytes::complete::tag, character, multi::separated_list1, sequence::separated_pair,
    IResult,
};
use std::cmp;
use std::fs;

struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, balls) = separated_list1(
            tag(", "),
            separated_pair(
                character::complete::u32,
                character::complete::char(' '),
                alt((tag("red"), tag("green"), tag("blue"))),
            ),
        )(input)?;
        let mut result = Self {
            red: 0,
            green: 0,
            blue: 0,
        };
        for (count, color) in balls {
            match color {
                "red" => result.red = count,
                "green" => result.green = count,
                "blue" => result.blue = count,
                _ => unreachable!(),
            }
        }
        Ok((input, result))
    }

    fn possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

struct Game {
    number: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Game ")(input)?;
        let (input, number) = character::complete::u32(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, draws) = separated_list1(tag("; "), Draw::parse)(input)?;
        Ok((input, Game { number, draws }))
    }
}

fn main() {
    let contents = fs::read_to_string("../02.txt").unwrap();
    pt1(&contents);
    pt2(&contents);
}

fn pt1(contents: &str) {
    let mut result: u32 = 0;
    for line in contents.lines() {
        let (_, game) = Game::parse(&line).unwrap();
        if game.draws.iter().all(|draw| draw.possible()) {
            result += game.number;
        }
    }
    println!("{}", result);
}

fn pt2(contents: &str) {
    let mut result: u32 = 0;
    for line in contents.lines() {
        let (_, game) = Game::parse(&line).unwrap();
        let mut acc = Draw {
            red: 0,
            green: 0,
            blue: 0,
        };
        for draw in &game.draws {
            acc.red = cmp::max(acc.red, draw.red);
            acc.green = cmp::max(acc.green, draw.green);
            acc.blue = cmp::max(acc.blue, draw.blue);
        }
        result += acc.red * acc.green * acc.blue;
    }
    println!("{}", result);
}
