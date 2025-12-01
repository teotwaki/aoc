use common::Answer;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, terminated},
};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Value {
    Item(i32),
    List(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Item(left), Self::Item(right)) => left.cmp(right),
            (Self::List(left), Self::List(right)) => {
                for (left, right) in left.iter().zip(right.iter()) {
                    let cmp = left.cmp(right);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                left.len().cmp(&right.len())
            }
            (Self::List(left), Self::Item(right)) => {
                let right = Vec::from([Value::Item(*right)]);
                Self::List(left.to_vec()).cmp(&Self::List(right))
            }
            (Self::Item(left), Self::List(right)) => {
                let left = Vec::from([Value::Item(*left)]);
                Self::List(left).cmp(&Self::List(right.to_vec()))
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Item(l0), Self::Item(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Value {}

fn parse_i32(s: &str) -> IResult<&str, i32> {
    map(digit1, |i: &str| i.parse().expect("Invalid number")).parse(s)
}

fn parse_value(s: &str) -> IResult<&str, Value> {
    delimited(
        tag("["),
        map(
            separated_list0(tag(","), alt((map(parse_i32, Value::Item), parse_value))),
            Value::List,
        ),
        tag("]"),
    )
    .parse(s)
}

fn parse_pair(s: &str) -> IResult<&str, (Value, Value)> {
    let (s, (v1, v2)) = (
        terminated(parse_value, newline),
        terminated(parse_value, newline),
    )
        .parse(s)?;

    Ok((s, (v1, v2)))
}

fn parse(s: &str) -> IResult<&str, Vec<(Value, Value)>> {
    separated_list1(newline, parse_pair).parse(s)
}

fn divider_packet(i: i32) -> Value {
    Value::List(vec![Value::List(vec![Value::Item(i)])])
}

pub fn step1(s: &str) -> Answer {
    let (_, pairs) = parse(s).expect("Couldn't parse input");

    let sum: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(i, _)| i + 1)
        .sum();

    sum.into()
}

pub fn step2(s: &str) -> Answer {
    let (_, pairs) = parse(s).expect("Couldn't parse input");

    let mut all_packets = pairs
        .into_iter()
        .flat_map(|t| [t.0, t.1])
        .collect::<Vec<Value>>();

    all_packets.push(divider_packet(2));
    all_packets.push(divider_packet(6));
    all_packets.sort_unstable();

    let decoder_key: usize = all_packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| *p == divider_packet(2) || *p == divider_packet(6))
        .map(|(i, _)| i + 1)
        .product();

    decoder_key.into()
}
