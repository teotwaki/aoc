use common::Answer;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::terminated,
};

#[derive(Debug, Clone)]
enum Operation {
    Add(i64),
    Multiply(i64),
    Square,
}

impl Operation {
    fn perform(&self, i: i64) -> i64 {
        match self {
            Operation::Add(n) => i + n,
            Operation::Multiply(n) => i * n,
            Operation::Square => i * i,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    op: Operation,
    divisible_by: i64,
    forward_to: (usize, usize),
    inspected: i64,
}

impl Monkey {
    fn process(
        &self,
        very_worried: bool,
        worry_divisor: i64,
    ) -> impl Iterator<Item = (i64, usize)> + '_ {
        self.items
            .iter()
            .map(|i| self.op.perform(*i))
            .map(move |i| {
                if very_worried {
                    i % worry_divisor
                } else {
                    i / worry_divisor
                }
            })
            .map(|i| {
                if i % self.divisible_by == 0 {
                    (i, self.forward_to.0)
                } else {
                    (i, self.forward_to.1)
                }
            })
    }

    fn clear(&mut self) {
        self.items.clear();
    }

    fn append(&mut self, item: i64) {
        self.items.push(item);
    }

    fn inspected(&mut self) {
        self.inspected += 1;
    }

    fn total_inspections(&self) -> i64 {
        self.inspected
    }
}

fn parse_usize(s: &str) -> IResult<&str, usize> {
    map(digit1, |i: &str| i.parse().expect("Invalid number")).parse(s)
}

fn parse_i64(s: &str) -> IResult<&str, i64> {
    map(digit1, |i: &str| i.parse().expect("Invalid number")).parse(s)
}

fn monkey_header(s: &str) -> IResult<&str, ()> {
    let (s, _) = terminated((tag("Monkey "), parse_usize, tag(":")), newline).parse(s)?;
    Ok((s, ()))
}

fn starting_items(s: &str) -> IResult<&str, Vec<i64>> {
    let (s, (_, items)) = terminated(
        (
            tag("  Starting items: "),
            separated_list1(tag(", "), parse_i64),
        ),
        newline,
    )
    .parse(s)?;

    Ok((s, items))
}

fn operation(s: &str) -> IResult<&str, Operation> {
    let (s, (_, op)) = terminated(
        (
            tag("  Operation: new = old "),
            alt((
                map(tag("* old"), |_| Operation::Square),
                map((tag("* "), parse_i64), |(_, i)| Operation::Multiply(i)),
                map((tag("+ "), parse_i64), |(_, i)| Operation::Add(i)),
            )),
        ),
        newline,
    )
    .parse(s)?;

    Ok((s, op))
}

fn divisible_by(s: &str) -> IResult<&str, i64> {
    let (s, (_, divisible_by)) =
        terminated((tag("  Test: divisible by "), parse_i64), newline).parse(s)?;

    Ok((s, divisible_by))
}

fn forward_to(s: &str) -> IResult<&str, (usize, usize)> {
    let (s, (_, true_monkey)) =
        terminated((tag("    If true: throw to monkey "), parse_usize), newline).parse(s)?;
    let (s, (_, false_monkey)) = terminated(
        (tag("    If false: throw to monkey "), parse_usize),
        newline,
    )
    .parse(s)?;

    Ok((s, (true_monkey, false_monkey)))
}

fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
    let (s, _) = monkey_header(s)?;
    let (s, items) = starting_items(s)?;
    let (s, op) = operation(s)?;
    let (s, divisible_by) = divisible_by(s)?;
    let (s, forward_to) = forward_to(s)?;

    Ok((
        s,
        Monkey {
            items,
            op,
            divisible_by,
            forward_to,
            inspected: 0,
        },
    ))
}

fn monkeys(s: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(newline, parse_monkey).parse(s)
}

fn play_round(monkeys: &mut [Monkey], very_worried: bool, worry_divisor: i64) {
    for i in 0..monkeys.len() {
        let monkey = monkeys[i].clone();

        monkey
            .process(very_worried, worry_divisor)
            .for_each(|(item, to_monkey)| {
                monkeys[to_monkey].append(item);
                monkeys[i].inspected();
            });

        monkeys[i].clear();
    }
}

pub fn step1(s: &str) -> Answer {
    let (_, mut monkeys) = monkeys(s).expect("Failed to parse monkey business");

    for _ in 0..20 {
        play_round(&mut monkeys, false, 3);
    }

    monkeys
        .iter()
        .map(|m| m.total_inspections())
        .sorted()
        .rev()
        .take(2)
        .product::<i64>()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let (_, mut monkeys) = monkeys(s).expect("Failed to parse monkey business");
    let gcd: i64 = monkeys.iter().map(|m| m.divisible_by).product();

    for _ in 0..10000 {
        play_round(&mut monkeys, true, gcd);
    }

    monkeys
        .iter()
        .map(|m| m.total_inspections())
        .sorted()
        .rev()
        .take(2)
        .product::<i64>()
        .into()
}
