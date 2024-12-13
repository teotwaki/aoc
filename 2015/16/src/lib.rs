use common::Answer;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{cmp::Ordering, sync::LazyLock};

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
    static PROPS: LazyLock<FxHashSet<(&str, IntType)>> = LazyLock::new(|| {
        let mut map = FxHashSet::default();

        map.insert(("children", 3));
        map.insert(("cats", 7));
        map.insert(("samoyeds", 2));
        map.insert(("pomeranians", 3));
        map.insert(("akitas", 0));
        map.insert(("vizslas", 0));
        map.insert(("goldfish", 5));
        map.insert(("trees", 3));
        map.insert(("cars", 2));
        map.insert(("perfumes", 1));

        map
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
    static PROPS: LazyLock<FxHashMap<&str, (Ordering, IntType)>> = LazyLock::new(|| {
        use Ordering::*;

        let mut map = FxHashMap::default();

        map.insert("children", (Equal, 3));
        map.insert("cats", (Greater, 7));
        map.insert("samoyeds", (Equal, 2));
        map.insert("pomeranians", (Less, 3));
        map.insert("akitas", (Equal, 0));
        map.insert("vizslas", (Equal, 0));
        map.insert("goldfish", (Less, 5));
        map.insert("trees", (Greater, 3));
        map.insert("cars", (Equal, 2));
        map.insert("perfumes", (Equal, 1));

        map
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
