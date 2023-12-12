use nom::{
    bytes::complete::tag,
    character::complete::{space1, u32 as nom_u32},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use std::fs;

const FILE_PATH: &str = "04.txt";

#[derive(Debug)]
struct Card {
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (winners, numbers)) = preceded(
            tuple((tag("Card"), space1, nom_u32, tag(":"), space1)),
            separated_pair(
                separated_list1(space1, nom_u32),
                tuple((space1, tag("|"), space1)),
                separated_list1(space1, nom_u32),
            ),
        )(input)?;
        Ok((input, Self { winners, numbers }))
    }

    fn matches(&self) -> usize {
        let mut result = 0;
        for number in &self.numbers {
            if self.winners.contains(number) {
                result += 1;
            }
        }
        result
    }

    fn total(&self) -> u32 {
        match self.matches() {
            0 => 0,
            v => {
                let mut result = 1;
                for _ in 1..v {
                    result <<= 1;
                }
                result
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string(FILE_PATH).unwrap();
    let mut cards: Vec<Card> = Vec::new();
    for line in input.lines() {
        let (_, card) = Card::parse(line).unwrap();
        cards.push(card);
    }
    pt1(&cards);
    pt2(&cards);
}

fn pt1(cards: &[Card]) {
    let mut result = 0;
    for card in cards {
        result += card.total();
    }
    println!("{}", result);
}

fn pt2(cards: &[Card]) {
    let mut card_counts = vec![1usize; cards.len()];
    for (index, card) in cards.iter().enumerate() {
        for offset in 1..=card.matches() {
            card_counts[index + offset] += card_counts[index];
        }
    }
    let result: usize = card_counts.iter().sum();
    println!("{}", result);
}
