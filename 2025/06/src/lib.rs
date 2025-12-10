use common::Answer;

type IntType = u64;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    pub fn apply(&self, terms: Vec<IntType>) -> IntType {
        match self {
            Operation::Add => terms.iter().sum(),
            Operation::Multiply => terms.iter().product(),
        }
    }
}

fn parse(s: &str) -> (Vec<Vec<IntType>>, Vec<Operation>) {
    let operations = s
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| match s {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let terms = s
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .skip(1)
        .rev()
        .map(|l| l.split_whitespace().map(|i| i.parse().unwrap()).collect())
        .collect();

    (terms, operations)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();

    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn do_math(terms: Vec<Vec<IntType>>, ops: Vec<Operation>) -> Answer {
    terms
        .into_iter()
        .zip(ops.iter())
        .map(|(terms, op)| op.apply(terms))
        .sum::<IntType>()
        .into()
}

pub fn step1(s: &str) -> Answer {
    let (terms, ops) = parse(s);
    let terms = transpose(terms);

    do_math(terms, ops)
}

struct CephTermReader<'a> {
    lines: &'a [Vec<char>; 4],
    pos: usize,
}

impl<'a> CephTermReader<'a> {
    pub fn new(lines: &'a [Vec<char>; 4], pos: usize) -> Self {
        Self { lines, pos }
    }

    fn at_end(&self) -> bool {
        self.pos == self.lines[0].len()
            || self
                .lines
                .iter()
                .all(|l| l.get(self.pos).is_none_or(|&c| c == ' '))
    }

    fn read_digit(&self, i: usize) -> Option<IntType> {
        self.lines[i]
            .get(self.pos)
            .and_then(|c| c.to_digit(10).map(|i| i as IntType))
    }
}

impl<'a> Iterator for CephTermReader<'a> {
    type Item = IntType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end() {
            return None;
        }

        let mut count = 0;
        let mut value = 0;

        for i in 0..4 {
            if let Some(d) = self.read_digit(i) {
                value += d * 10u64.pow(count);
                count += 1;
            }
        }

        self.pos += 1;

        Some(value)
    }
}

struct CephMathReader {
    lines: [Vec<char>; 4],
    ops: Vec<char>,
    pos: usize,
}

impl CephMathReader {
    pub fn new(lines: &[&str]) -> Self {
        Self {
            lines: [
                lines[3].chars().collect(),
                lines[2].chars().collect(),
                lines[1].chars().collect(),
                lines[0].chars().collect(),
            ],
            ops: lines.last().unwrap().chars().collect(),
            pos: 0,
        }
    }

    fn at_end(&self) -> bool {
        self.pos >= self.ops.len()
    }
}

impl Iterator for CephMathReader {
    type Item = (Operation, Vec<IntType>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end() {
            return None;
        }

        let op = if self.ops[self.pos] == '*' {
            Operation::Multiply
        } else {
            Operation::Add
        };

        let terms = CephTermReader::new(&self.lines, self.pos).collect::<Vec<_>>();

        self.pos += terms.len();

        if !self.at_end() {
            self.pos += 1;
        }

        Some((op, terms))
    }
}

pub fn step2(s: &str) -> Answer {
    let lines = s.lines().collect::<Vec<_>>();
    let (ops, terms) = CephMathReader::new(&lines).collect::<(Vec<Operation>, Vec<Vec<IntType>>)>();

    do_math(terms, ops)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +   "#;

    #[test]
    fn step1_finds_correct_sample_value() {
        assert_eq!(step1(INPUT), Answer::Unsigned(4277556));
    }

    #[test]
    fn step2_finds_correct_sample_value() {
        assert_eq!(step2(INPUT), Answer::Unsigned(3263827));
    }
}
