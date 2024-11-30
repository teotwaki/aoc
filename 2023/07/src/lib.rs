use common::Answer;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => unreachable!(),
        }
    }
}

impl Card {
    fn to_joker(self) -> Self {
        use Card::*;

        match self {
            Jack => Joker,
            c => c,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Hand {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&Vec<Card>> for Hand {
    fn from(cards: &Vec<Card>) -> Self {
        let mut freq: Vec<_> = cards
            .iter()
            .fold(HashMap::<Card, usize>::new(), |mut map, card| {
                *map.entry(*card).or_default() += 1;
                map
            })
            .into_iter()
            .collect();

        freq.sort_by(|a, b| match b.1.cmp(&a.1) {
            Ordering::Equal => b.0.cmp(&a.0),
            ord => ord,
        });

        for i in 0..freq.len() {
            if freq[i].0 == Card::Joker {
                if freq.len() == 1 {
                    break;
                }

                let dst = if i == 0 { 1 } else { 0 };

                freq[dst].1 += freq[i].1;
                freq.remove(i);
                break;
            }
        }

        match freq.len() {
            5 => Hand::HighCard,
            4 => Hand::Pair,
            3 if freq[0].1 == 2 => Hand::TwoPairs,
            3 if freq[0].1 == 3 => Hand::ThreeOfAKind,
            2 if freq[0].1 == 3 => Hand::FullHouse,
            2 => Hand::FourOfAKind,
            1 => Hand::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Round {
    cards: Vec<Card>,
    hand: Hand,
    bid: u16,
}

pub fn step1(s: &str) -> Answer {
    let mut rounds: Vec<_> = s
        .lines()
        .map(|l| {
            let cards = l[..5].chars().map(Card::from).collect();

            Round {
                hand: Hand::from(&cards),
                bid: l[6..].parse().unwrap(),
                cards,
            }
        })
        .collect();

    rounds.sort_by(|a, b| match a.hand.cmp(&b.hand) {
        Ordering::Equal => a.cards.cmp(&b.cards),
        ord => ord,
    });

    rounds
        .iter()
        .enumerate()
        .map(|(i, r)| (i + 1) * r.bid as usize)
        .sum::<usize>()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let mut rounds: Vec<_> = s
        .lines()
        .map(|l| {
            let cards = l[..5].chars().map(|c| Card::from(c).to_joker()).collect();

            Round {
                hand: Hand::from(&cards),
                bid: l[6..].parse().unwrap(),
                cards,
            }
        })
        .collect();

    rounds.sort_by(|a, b| match a.hand.cmp(&b.hand) {
        Ordering::Equal => a.cards.cmp(&b.cards),
        ord => ord,
    });

    rounds
        .iter()
        .enumerate()
        .map(|(i, r)| (i + 1) * r.bid as usize)
        .sum::<usize>()
        .into()
}
