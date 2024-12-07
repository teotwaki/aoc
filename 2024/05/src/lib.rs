use common::Answer;
use rayon::prelude::*;
use std::{cmp::Ordering, collections::HashMap};

type IntType = u16;

struct Rulebook {
    before: HashMap<IntType, Vec<IntType>>,
}

impl Rulebook {
    fn new() -> Self {
        Self {
            before: HashMap::new(),
        }
    }

    fn insert(&mut self, a: IntType, b: IntType) {
        self.before
            .entry(a)
            .and_modify(|v| v.push(b))
            .or_insert_with(|| vec![b]);
    }

    fn is_before(&self, a: IntType, b: IntType) -> bool {
        self.before.get(&a).map(|v| v.contains(&b)).unwrap_or(false)
    }

    fn compare(&self, a: IntType, b: IntType) -> Ordering {
        if self.is_before(a, b) {
            Ordering::Less
        } else if self.is_before(b, a) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

fn parse(s: &str) -> (Rulebook, Vec<Vec<IntType>>) {
    let mut parts = s.split("\n\n");
    let (rules, updates) = (parts.next().unwrap(), parts.next().unwrap());

    let mut rulebook = Rulebook::new();

    rules
        .lines()
        .map(|s| {
            let mut parts = s.split('|');
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .for_each(|(a, b)| rulebook.insert(a, b));

    let updates = updates
        .lines()
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    (rulebook, updates)
}

fn validate(update: &[IntType], rules: &Rulebook) -> bool {
    update.windows(2).all(|w| rules.is_before(w[0], w[1]))
}

pub fn step1(s: &str) -> Answer {
    let (rulebook, updates) = parse(s);
    updates
        .iter()
        .filter(|u| validate(u, &rulebook))
        .map(|u| u[u.len() / 2])
        .sum::<IntType>()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let (rulebook, mut updates) = parse(s);

    updates
        .par_iter_mut()
        .filter(|u| !validate(u, &rulebook))
        .map(|u| {
            u.sort_by(|&a, &b| rulebook.compare(a, b));
            u[u.len() / 2]
        })
        .sum::<IntType>()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn step1_finds_correct_value() {
        assert_eq!(step1(INPUT), Answer::Unsigned(143));
    }

    #[test]
    fn step2_finds_correct_value() {
        assert_eq!(step2(INPUT), Answer::Unsigned(123));
    }
}
