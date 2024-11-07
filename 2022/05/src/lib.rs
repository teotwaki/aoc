use common::Answer;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated, tuple},
    IResult,
};
use std::collections::VecDeque;

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Document {
    columns: Vec<VecDeque<char>>,
    instructions: Vec<Instruction>,
}

fn top_crates(columns: Vec<VecDeque<char>>) -> String {
    columns.iter().map(|c| c[0]).collect()
}

impl Document {
    pub fn simulate_9000(&self) -> String {
        let mut columns = self.columns.clone();

        for instruction in &self.instructions {
            for _ in 0..(instruction.count) {
                if let Some(c) = columns[instruction.from].pop_front() {
                    columns[instruction.to].push_front(c);
                }
            }
        }

        top_crates(columns)
    }

    pub fn simulate_9001(&self) -> String {
        let mut columns = self.columns.clone();

        for instruction in &self.instructions {
            let crates: Vec<char> = columns[instruction.from]
                .drain(0..instruction.count)
                .collect();
            crates
                .iter()
                .enumerate()
                .for_each(|(i, c)| columns[instruction.to].insert(i, *c));
        }

        top_crates(columns)
    }
}

fn parse_crate(s: &str) -> IResult<&str, Option<char>> {
    let (s, c) = delimited(tag("["), anychar, tag("]"))(s)?;
    Ok((s, Some(c)))
}

fn parse_no_crate(s: &str) -> IResult<&str, Option<char>> {
    let (s, _) = tuple((tag(" "), tag(" "), tag(" ")))(s)?;
    Ok((s, None))
}

fn crate_line(s: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(tag(" "), alt((parse_crate, parse_no_crate)))(s)
}

fn crate_lines(s: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    many1(terminated(crate_line, tag("\n")))(s)
}

fn parse_usize(s: &str) -> IResult<&str, usize> {
    use nom::combinator::map;
    map(digit1, |i: &str| {
        i.parse::<usize>().expect("Invalid number")
    })(s)
}

fn columns_line(s: &str) -> IResult<&str, Vec<usize>> {
    terminated(
        delimited(
            tag(" "),
            separated_list1(many1(tag(" ")), parse_usize),
            tag(" "),
        ),
        tag("\n"),
    )(s)
}

fn instruction_line(s: &str) -> IResult<&str, Instruction> {
    let (s, (_, count, _, from, _, to)) = tuple((
        tag("move "),
        parse_usize,
        tag(" from "),
        parse_usize,
        tag(" to "),
        parse_usize,
    ))(s)?;

    Ok((
        s,
        Instruction {
            count,
            from: from - 1,
            to: to - 1,
        },
    ))
}

fn instruction_lines(s: &str) -> IResult<&str, Vec<Instruction>> {
    many1(terminated(instruction_line, tag("\n")))(s)
}

fn transform_crates(column_count: usize, lines: Vec<Vec<Option<char>>>) -> Vec<VecDeque<char>> {
    lines
        .iter()
        .fold(vec![VecDeque::new(); column_count], |mut columns, line| {
            for i in 0..column_count {
                if let Some(c) = line[i] {
                    columns[i].push_back(c);
                }
            }
            columns
        })
}

fn parse(s: &str) -> IResult<&str, Document> {
    let (s, crate_lines) = crate_lines(s)?;
    let (s, columns) = columns_line(s)?;
    let column_count = columns.last().expect("No column count detected");
    let crate_columns = transform_crates(*column_count, crate_lines);
    let (s, _) = tag("\n")(s)?;
    let (s, instructions) = instruction_lines(s)?;

    Ok((
        s,
        Document {
            columns: crate_columns,
            instructions,
        },
    ))
}

pub fn step1(s: &str) -> Answer {
    let (_, document) = parse(s).unwrap();

    document.simulate_9000().into()
}

pub fn step2(s: &str) -> Answer {
    let (_, document) = parse(s).unwrap();

    document.simulate_9001().into()
}
