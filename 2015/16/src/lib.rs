use common::Answer;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

type IntType = u8;

fn parse(s: &str) -> impl Iterator<Item = Vec<(&str, IntType)>> {
    s.lines().map(|l| {
        let mut parts = l.split_whitespace().skip(2);

        (0..3)
            .map(|_| {
                let property = parts.next().map(|s| s.trim_matches(':')).unwrap();
                let value = parts
                    .next()
                    .and_then(|s| s.trim_matches(',').parse().ok())
                    .unwrap();

                (property, value)
            })
            .collect()
    })
}

fn matches_aunt_exactly(aunt_props: &[(&str, IntType)]) -> bool {
    static PROPS: LazyLock<HashSet<(&str, IntType)>> = LazyLock::new(|| {
        HashSet::from([
            ("children", 3),
            ("cats", 7),
            ("samoyeds", 2),
            ("pomeranians", 3),
            ("akitas", 0),
            ("vizslas", 0),
            ("goldfish", 5),
            ("trees", 3),
            ("cars", 2),
            ("perfumes", 1),
        ])
    });

    aunt_props.iter().all(|prop| PROPS.contains(prop))
}

fn find_aunt<F: Fn(&[(&str, IntType)]) -> bool>(s: &str, f: F) -> usize {
    parse(s)
        .enumerate()
        .filter(|(_, props)| f(props))
        .map(|(i, _)| i + 1)
        .next()
        .unwrap()
}

pub fn step1(s: &str) -> Answer {
    find_aunt(s, matches_aunt_exactly).into()
}

fn matches_aunt_range(aunt_props: &[(&str, IntType)]) -> bool {
    static PROPS: LazyLock<HashMap<&str, (Ordering, IntType)>> = LazyLock::new(|| {
        use Ordering::*;

        HashMap::from([
            ("children", (Equal, 3)),
            ("cats", (Greater, 7)),
            ("samoyeds", (Equal, 2)),
            ("pomeranians", (Less, 3)),
            ("akitas", (Equal, 0)),
            ("vizslas", (Equal, 0)),
            ("goldfish", (Less, 5)),
            ("trees", (Greater, 3)),
            ("cars", (Equal, 2)),
            ("perfumes", (Equal, 1)),
        ])
    });

    aunt_props
        .iter()
        .all(|&(name, value)| match PROPS.get(name) {
            Some((ord, i)) => value.cmp(i) == *ord,
            _ => true,
        })
}

pub fn step2(s: &str) -> Answer {
    find_aunt(s, matches_aunt_range).into()
}
