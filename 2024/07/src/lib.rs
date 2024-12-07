use std::iter::{repeat_n, successors};

use common::Answer;
use itertools::Itertools;

type IntType = u64;

fn parse(s: &str) -> impl Iterator<Item = (IntType, Vec<IntType>)> + '_ {
    s.lines().map(|l| {
        let mut parts = l.split(':');
        let result = parts.next().unwrap().parse().unwrap();
        let terms = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        (result, terms)
    })
}

fn int_len(n: IntType) -> u32 {
    successors(Some(n), |&n| (n >= 10).then_some(n / 10)).count() as u32
}

fn apply(op: &str, a: IntType, b: IntType) -> IntType {
    match op {
        "add" => a + b,
        "mul" => a * b,
        "concat" => a * 10u64.pow(int_len(b)) + b,
        _ => unreachable!(),
    }
}

fn is_solvable(ops: &[&str], result: IntType, terms: &[IntType]) -> bool {
    repeat_n(ops.iter(), terms.len() - 1)
        .multi_cartesian_product()
        .any(|ops| {
            terms[1..]
                .iter()
                .zip(ops)
                .fold(terms[0], |acc, (term, op)| apply(op, acc, *term))
                == result
        })
}

fn sum_solvable(ops: &[&str], s: &str) -> IntType {
    parse(s)
        .filter(|(r, t)| is_solvable(ops, *r, t))
        .map(|(result, _)| result)
        .sum::<IntType>()
}

pub fn step1(s: &str) -> Answer {
    sum_solvable(&["add", "mul"], s).into()
}

pub fn step2(s: &str) -> Answer {
    sum_solvable(&["add", "mul", "concat"], s).into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn step1_finds_correct_answer() {
        assert_eq!(step1(INPUT), Answer::Unsigned(3749));
    }
}
