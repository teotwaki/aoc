use common::Answer;

#[derive(Copy, Clone, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

struct OpponentMove(char);
struct MyMove(char);
struct Hand(Shape, Shape);
struct StrategicHand(Shape, Outcome);

impl From<OpponentMove> for Shape {
    fn from(m: OpponentMove) -> Self {
        match m.0 {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => unreachable!(),
        }
    }
}

impl From<MyMove> for Shape {
    fn from(m: MyMove) -> Self {
        match m.0 {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq)]
enum Outcome {
    Victory = 6,
    Draw = 3,
    Loss = 0,
}

impl From<Shape> for Outcome {
    fn from(s: Shape) -> Self {
        match s {
            Shape::Rock => Outcome::Loss,
            Shape::Paper => Outcome::Draw,
            Shape::Scissors => Outcome::Victory,
        }
    }
}

impl From<&Hand> for Outcome {
    fn from(hand: &Hand) -> Self {
        if hand.0 == hand.1 {
            Outcome::Draw
        } else {
            match (hand.0, hand.1) {
                (Shape::Rock, Shape::Paper) => Outcome::Victory,
                (Shape::Rock, Shape::Scissors) => Outcome::Loss,
                (Shape::Paper, Shape::Rock) => Outcome::Loss,
                (Shape::Paper, Shape::Scissors) => Outcome::Victory,
                (Shape::Scissors, Shape::Rock) => Outcome::Victory,
                (Shape::Scissors, Shape::Paper) => Outcome::Loss,
                _ => unreachable!(),
            }
        }
    }
}

impl From<StrategicHand> for Hand {
    fn from(hand: StrategicHand) -> Self {
        if hand.1 == Outcome::Draw {
            Hand(hand.0, hand.0)
        } else {
            match (hand.0, hand.1) {
                (Shape::Rock, Outcome::Victory) => Hand(Shape::Rock, Shape::Paper),
                (Shape::Rock, Outcome::Loss) => Hand(Shape::Rock, Shape::Scissors),
                (Shape::Paper, Outcome::Victory) => Hand(Shape::Paper, Shape::Scissors),
                (Shape::Paper, Outcome::Loss) => Hand(Shape::Paper, Shape::Rock),
                (Shape::Scissors, Outcome::Victory) => Hand(Shape::Scissors, Shape::Rock),
                (Shape::Scissors, Outcome::Loss) => Hand(Shape::Scissors, Shape::Paper),
                _ => unreachable!(),
            }
        }
    }
}

fn score(hand: &Hand) -> i32 {
    Outcome::from(hand) as i32 + hand.1 as i32
}

fn get_hands(s: &str) -> Vec<Hand> {
    s.lines()
        .map(|l| (l.chars().next().unwrap(), l.chars().nth(2).unwrap()))
        .map(|(om, mm)| (OpponentMove(om), MyMove(mm)))
        .map(|(om, mm)| Hand(om.into(), mm.into()))
        .collect()
}

pub fn step1(s: &str) -> Answer {
    get_hands(s).iter().map(score).sum::<i32>().into()
}

pub fn step2(s: &str) -> Answer {
    get_hands(s)
        .iter()
        .map(|hand| StrategicHand(hand.0, hand.1.into()))
        .map(|hand| score(&hand.into()))
        .sum::<i32>()
        .into()
}
