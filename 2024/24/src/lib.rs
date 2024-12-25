use common::Answer;
use rustc_hash::FxHashMap;

type Wires<'a> = FxHashMap<&'a str, bool>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    fn output(&self, a: bool, b: bool) -> bool {
        use Gate::*;

        match self {
            And => (a as u8 & b as u8) == 1,
            Or => (a as u8 | b as u8) == 1,
            Xor => (a as u8 ^ b as u8) == 1,
        }
    }
}

fn parse_wires(s: &str) -> Wires {
    s.lines()
        .map(|line| {
            let mut parts = line.split(": ");

            (parts.next().unwrap(), parts.next().unwrap() == "1")
        })
        .collect()
}

fn parse_gates(s: &str) -> Vec<(Gate, &str, &str, &str)> {
    s.lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let a = parts.next().unwrap();
            let gate = parts.next().unwrap();
            let b = parts.next().unwrap();
            let c = parts.nth(1).unwrap();

            use Gate::*;

            match gate {
                "AND" => (And, a, b, c),
                "OR" => (Or, a, b, c),
                "XOR" => (Xor, a, b, c),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn parse(s: &str) -> (Wires, Vec<(Gate, &str, &str, &str)>) {
    let mut parts = s.split("\n\n");

    (
        parse_wires(parts.next().unwrap()),
        parse_gates(parts.next().unwrap()),
    )
}

pub fn step1(s: &str) -> Answer {
    let (mut wires, mut gates) = parse(s);

    'w: while !gates.is_empty() {
        for i in 0..gates.len() {
            let (gate, a, b, c) = gates[i];

            if let (Some(a), Some(b)) = (wires.get(a), wires.get(b)) {
                wires.insert(c, gate.output(*a, *b));
                gates.remove(i);
                continue 'w;
            }
        }
    }

    wires.retain(|key, _| key.starts_with('z'));
    let mut keys = wires.keys().collect::<Vec<_>>();
    keys.sort();

    keys.iter()
        .enumerate()
        .filter_map(|(n, k)| {
            if *wires.get(*k).unwrap_or(&false) {
                Some(1 << n)
            } else {
                None
            }
        })
        .sum::<usize>()
        .into()
}

pub fn step2(_: &str) -> Answer {
    ().into()
}
