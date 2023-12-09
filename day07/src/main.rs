use nom::{character, character::complete::one_of, multi::count, IResult};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::iter;

#[derive(Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
enum Card {
    Joker,
    Other(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn parse(input: &str, jack_or_joker: fn() -> Card) -> IResult<&str, Self> {
        let (input, character) = one_of("AKQJT98765432")(input)?;
        let card: Card = match character {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => jack_or_joker(),
            'T' => Card::Other(10),
            _ => Card::Other(character.to_digit(10).unwrap().try_into().unwrap()),
        };
        Ok((input, card))
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn parse(input: &str, jack_or_joker: fn() -> Card) -> IResult<&str, Self> {
        let (input, cards) = count(|i| Card::parse(i, jack_or_joker), 5)(input)?;
        let (input, _) = character::complete::char(' ')(input)?;
        let (input, bid) = character::complete::u32(input)?;
        Ok((
            input,
            Hand {
                cards: cards.try_into().unwrap(),
                bid,
            },
        ))
    }

    fn hand_type(&self) -> HandType {
        let mut card_counts = HashMap::<&Card, u32>::new();
        for card in &self.cards {
            *card_counts.entry(card).or_insert(0) += 1;
        }
        let joker_count = card_counts.remove(&Card::Joker).unwrap_or_default();
        if joker_count == 5 {
            return HandType::FiveOfAKind;
        }
        let mut pair_count = 0;
        let mut trio_count = 0;
        for (_, count) in card_counts {
            if count + joker_count == 5 {
                return HandType::FiveOfAKind;
            }
            if count + joker_count == 4 {
                return HandType::FourOfAKind;
            }
            match count {
                3 => trio_count += 1,
                2 => pair_count += 1,
                _ => (),
            }
        }
        // If you have a trio, you can't have a joker.
        if trio_count == 1 && pair_count == 1 {
            return HandType::FullHouse;
        }
        if trio_count == 1 {
            return HandType::ThreeOfAKind;
        }
        // If you have a pair, you could have at most one joker.
        if pair_count == 2 && joker_count == 1 {
            return HandType::FullHouse;
        }
        if pair_count == 2 {
            return HandType::TwoPair;
        }
        if pair_count == 1 && joker_count == 1 {
            return HandType::ThreeOfAKind;
        }
        if pair_count == 1 {
            return HandType::OnePair;
        }
        // If you have single cards, you can have two jokers.
        match joker_count {
            2 => HandType::ThreeOfAKind,
            1 => HandType::OnePair,
            0 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let contents = fs::read_to_string("07.txt").unwrap();
    run(&contents, || Card::Jack);
    run(&contents, || Card::Joker);
}

fn cmp_cards(lhs: &[Card; 5], rhs: &[Card; 5]) -> Ordering {
    for (c1, c2) in iter::zip(lhs, rhs) {
        match c1.cmp(c2) {
            Ordering::Equal => continue,
            result => return result,
        }
    }
    Ordering::Equal
}

fn run(contents: &str, jack_or_joker: fn() -> Card) {
    let mut result: u32 = 0;
    let hands: Vec<Hand> = contents
        .lines()
        .map(|l| Hand::parse(l, jack_or_joker).unwrap().1)
        .collect();
    let mut hand_types: Vec<(HandType, &Hand)> = hands.iter().map(|h| (h.hand_type(), h)).collect();
    hand_types.sort_unstable_by(|(t1, h1), (t2, h2)| {
        t1.cmp(t2).then_with(|| cmp_cards(&h1.cards, &h2.cards))
    });
    for (index, (_hand_type, hand)) in hand_types.iter().enumerate() {
        let rank: u32 = (index + 1).try_into().unwrap();
        result += rank * hand.bid;
    }
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_cmp() {
        assert_eq!(Card::Ace.cmp(&Card::Other(10)), Ordering::Greater);
        assert_eq!(Card::Other(10).cmp(&Card::Other(8)), Ordering::Greater);
    }
}
