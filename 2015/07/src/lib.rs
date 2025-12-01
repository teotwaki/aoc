use common::Answer;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u16 as parse_u16},
    multi::separated_list1,
};
use rustc_hash::FxHashMap;

type IntType = u16;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Wire<'a>(&'a str);

#[derive(Debug, Clone, Copy)]
enum Value<'a> {
    Wire(Wire<'a>),
    Value(IntType),
}

#[derive(Debug, Clone, Copy)]
enum Operation<'a> {
    Assign(Value<'a>, Wire<'a>),
    And(Value<'a>, Value<'a>, Wire<'a>),
    Or(Value<'a>, Value<'a>, Wire<'a>),
    Lshift(Value<'a>, IntType, Wire<'a>),
    Rshift(Value<'a>, IntType, Wire<'a>),
    Not(Value<'a>, Wire<'a>),
}

fn parse_wire(s: &str) -> IResult<&str, Wire<'_>> {
    let (s, w) = alpha1(s)?;

    Ok((s, Wire(w)))
}

fn parse_value_w(s: &str) -> IResult<&str, Value<'_>> {
    let (s, wire) = parse_wire(s)?;

    Ok((s, Value::Wire(wire)))
}

fn parse_value_u16(s: &str) -> IResult<&str, Value<'_>> {
    let (s, val) = parse_u16(s)?;

    Ok((s, Value::Value(val)))
}

fn parse_value(s: &str) -> IResult<&str, Value<'_>> {
    alt((parse_value_w, parse_value_u16)).parse(s)
}

fn parse_assign(s: &str) -> IResult<&str, Operation<'_>> {
    let (s, (value, _, dst)) = (parse_value, tag(" -> "), parse_wire).parse(s)?;

    Ok((s, Operation::Assign(value, dst)))
}

fn parse_and(s: &str) -> IResult<&str, Operation<'_>> {
    let (s, (a, _, b, _, dst)) = (
        parse_value,
        tag(" AND "),
        parse_value,
        tag(" -> "),
        parse_wire,
    )
        .parse(s)?;

    Ok((s, Operation::And(a, b, dst)))
}

fn parse_or(s: &str) -> IResult<&str, Operation<'_>> {
    let (s, (a, _, b, _, dst)) = (
        parse_value,
        tag(" OR "),
        parse_value,
        tag(" -> "),
        parse_wire,
    )
        .parse(s)?;

    Ok((s, Operation::Or(a, b, dst)))
}

fn parse_lshift(s: &str) -> IResult<&str, Operation<'_>> {
    let (s, (src, _, shift, _, dst)) = (
        parse_value,
        tag(" LSHIFT "),
        parse_u16,
        tag(" -> "),
        parse_wire,
    )
        .parse(s)?;

    Ok((s, Operation::Lshift(src, shift, dst)))
}

fn parse_rshift(s: &str) -> IResult<&str, Operation<'_>> {
    let (s, (src, _, shift, _, dst)) = (
        parse_value,
        tag(" RSHIFT "),
        parse_u16,
        tag(" -> "),
        parse_wire,
    )
        .parse(s)?;

    Ok((s, Operation::Rshift(src, shift, dst)))
}

fn parse_not(s: &str) -> IResult<&str, Operation<'_>> {
    let (s, (_, src, _, dst)) = (tag("NOT "), parse_value, tag(" -> "), parse_wire).parse(s)?;

    Ok((s, Operation::Not(src, dst)))
}

fn parse_operation(s: &str) -> IResult<&str, Operation<'_>> {
    alt((
        parse_assign,
        parse_and,
        parse_or,
        parse_lshift,
        parse_rshift,
        parse_not,
    ))
    .parse(s)
}

fn parse_operations(s: &str) -> IResult<&str, Vec<Operation<'_>>> {
    separated_list1(tag("\n"), parse_operation).parse(s)
}

fn parse(s: &str) -> Vec<Operation<'_>> {
    let (_, operations) = parse_operations(s).unwrap();

    operations
}

#[derive(Debug, Clone)]
struct Circuit<'a> {
    wires: FxHashMap<Wire<'a>, IntType>,
}

impl<'a> Circuit<'a> {
    fn new() -> Self {
        Self {
            wires: FxHashMap::default(),
        }
    }

    #[inline]
    fn and(&mut self, a: Value<'a>, b: Value<'a>, dst: Wire<'a>) -> Option<()> {
        let value = self.get_value(a)? & self.get_value(b)?;
        self.wires.insert(dst, value);

        Some(())
    }

    #[inline]
    fn or(&mut self, a: Value<'a>, b: Value<'a>, dst: Wire<'a>) -> Option<()> {
        let value = self.get_value(a)? | self.get_value(b)?;
        self.wires.insert(dst, value);

        Some(())
    }

    #[inline]
    fn lshift(&mut self, src: Value<'a>, shift: IntType, dst: Wire<'a>) -> Option<()> {
        let value = self.get_value(src)? << shift;
        self.wires.insert(dst, value);

        Some(())
    }

    #[inline]
    fn rshift(&mut self, src: Value<'a>, shift: IntType, dst: Wire<'a>) -> Option<()> {
        let value = self.get_value(src)? >> shift;
        self.wires.insert(dst, value);

        Some(())
    }

    #[inline]
    fn not(&mut self, src: Value<'a>, dst: Wire<'a>) -> Option<()> {
        let value = !self.get_value(src)?;
        self.wires.insert(dst, value);

        Some(())
    }

    fn process(&mut self, op: Operation<'a>) -> bool {
        match op {
            Operation::Assign(value, dst) => self
                .get_value(value)
                .map(|v| self.wires.insert(dst, v))
                .map(|_| ()),
            Operation::And(a, b, dst) => self.and(a, b, dst),
            Operation::Or(a, b, dst) => self.or(a, b, dst),
            Operation::Lshift(src, shift, dst) => self.lshift(src, shift, dst),
            Operation::Rshift(src, shift, dst) => self.rshift(src, shift, dst),
            Operation::Not(src, dst) => self.not(src, dst),
        }
        .is_none()
    }

    fn get_value(&self, v: Value<'a>) -> Option<IntType> {
        match v {
            Value::Value(v) => Some(v),
            Value::Wire(w) => self.get(w),
        }
    }

    #[inline]
    fn get(&self, k: Wire<'a>) -> Option<IntType> {
        self.wires.get(&k).copied()
    }
}

fn simulate(s: &str) -> Circuit<'_> {
    let mut circuit = Circuit::new();
    let mut operations = parse(s);

    while !operations.is_empty() {
        operations.retain(|&op| circuit.process(op));
    }

    circuit
}

pub fn step1(s: &str) -> Answer {
    let circuit = simulate(s);

    circuit.get(Wire("a")).unwrap().into()
}

pub fn step2(s: &str) -> Answer {
    let answer = step1(s);

    let input = s
        .lines()
        .map(|l| {
            if l.ends_with("-> b") {
                format!("{} -> b\n", answer)
            } else {
                format!("{}\n", l)
            }
        })
        .collect::<String>();

    step1(input.trim())
}

#[cfg(test)]
mod test_2015_07 {
    use super::*;

    const INPUT: &str = r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"#;

    #[test]
    fn parse_finds_correct_number_of_operations() {
        assert_eq!(parse(INPUT).len(), 8);
    }

    #[test]
    fn circuit_correctly_simulates_wires() {
        let circuit = simulate(INPUT);

        [
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ]
        .into_iter()
        .for_each(|(k, v)| assert_eq!(circuit.get(Wire(k)), Some(v)));
    }
}
