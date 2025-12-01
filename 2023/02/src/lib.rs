use common::Answer;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map,
    multi::separated_list1,
};
use std::cmp::max;

enum Cubes {
    Red(u8),
    Green(u8),
    Blue(u8),
}

#[derive(Clone, Copy, Debug, Default)]
struct Hand {
    red: u8,
    blue: u8,
    green: u8,
}

#[derive(Clone, Debug)]
struct Game {
    id: u8,
    hands: Vec<Hand>,
}

impl Game {
    fn is_possible(&self) -> bool {
        for hand in &self.hands {
            if hand.red > 12 || hand.green > 13 || hand.blue > 14 {
                return false;
            }
        }

        true
    }

    fn minimum_cubes(&self) -> (u8, u8, u8) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for hand in &self.hands {
            red = max(red, hand.red);
            green = max(green, hand.green);
            blue = max(blue, hand.blue);
        }

        (red, green, blue)
    }

    fn power(&self) -> u32 {
        let mins = self.minimum_cubes();

        mins.0 as u32 * mins.1 as u32 * mins.2 as u32
    }
}

fn parse_u8(s: &str) -> IResult<&str, u8> {
    map(digit1, |i: &str| i.parse().expect("Invalid number")).parse(s)
}

fn parse_red(s: &str) -> IResult<&str, Cubes> {
    let (s, (n, _)) = (parse_u8, tag(" red")).parse(s)?;

    Ok((s, Cubes::Red(n)))
}

fn parse_green(s: &str) -> IResult<&str, Cubes> {
    let (s, (n, _)) = (parse_u8, tag(" green")).parse(s)?;

    Ok((s, Cubes::Green(n)))
}

fn parse_blue(s: &str) -> IResult<&str, Cubes> {
    let (s, (n, _)) = (parse_u8, tag(" blue")).parse(s)?;

    Ok((s, Cubes::Blue(n)))
}

fn parse_cubes(s: &str) -> IResult<&str, Cubes> {
    alt((parse_red, parse_green, parse_blue)).parse(s)
}

fn parse_hand(s: &str) -> IResult<&str, Hand> {
    let (s, presented_cubes) = separated_list1(tag(", "), parse_cubes).parse(s)?;
    let mut h = Hand::default();

    for cubes in presented_cubes {
        match cubes {
            Cubes::Red(n) => h.red = n,
            Cubes::Green(n) => h.green = n,
            Cubes::Blue(n) => h.blue = n,
        }
    }

    Ok((s, h))
}

fn parse_game(s: &str) -> IResult<&str, Game> {
    let (s, (_, game_id, _)) = (tag("Game "), parse_u8, tag(": ")).parse(s)?;
    let (s, hands) = separated_list1(tag("; "), parse_hand).parse(s)?;

    Ok((s, Game { id: game_id, hands }))
}

fn parse(s: &str) -> Vec<Game> {
    let (_, games) = separated_list1(newline, parse_game)
        .parse(s)
        .expect("Failed to parse lines");

    games
}

pub fn step1(s: &str) -> Answer {
    let games = parse(s);

    games
        .iter()
        .filter_map(|g| if g.is_possible() { Some(g.id) } else { None })
        .map(|id| id as u32)
        .sum::<u32>()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let games = parse(s);

    games.iter().map(|g| g.power()).sum::<u32>().into()
}
