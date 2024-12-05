use common::Answer;
use itertools::Itertools;
use std::collections::HashMap;

type IntType = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location<'a>(&'a str);

#[derive(Debug, Clone, Copy)]
struct Distance(IntType);

#[derive(Debug, Clone, Copy)]
struct Route<'a>(Location<'a>, Location<'a>, Distance);

fn parse(s: &str) -> impl Iterator<Item = Route> {
    s.lines().map(|l| {
        let mut parts = l.split_whitespace();
        let src = Location(parts.next().unwrap());
        parts.next();
        let dst = Location(parts.next().unwrap());
        parts.next();
        let dist = Distance(
            parts
                .next()
                .and_then(|s| s.parse::<IntType>().ok())
                .unwrap(),
        );

        Route(src, dst, dist)
    })
}

#[derive(Debug, Clone)]
struct Graph<'a>(HashMap<Location<'a>, HashMap<Location<'a>, Distance>>);

impl<'a> Graph<'a> {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, a: Location<'a>, b: Location<'a>, distance: Distance) {
        self.0.entry(a).or_default().insert(b, distance);
        self.0.entry(b).or_default().insert(a, distance);
    }

    fn locations(&self) -> impl Iterator<Item = Location<'a>> + '_ {
        self.0.keys().copied()
    }

    fn get(&self, a: Location<'a>, b: Location<'a>) -> Option<Distance> {
        self.0.get(&a).and_then(|h| h.get(&b)).copied()
    }
}

fn distances(s: &str) -> Vec<IntType> {
    let mut graph = Graph::new();

    parse(s).for_each(|r| graph.insert(r.0, r.1, r.2));

    let locations_count = graph.locations().count();
    graph
        .locations()
        .permutations(locations_count)
        .map(|locs| {
            locs.windows(2)
                .map(|w| graph.get(w[0], w[1]).unwrap().0)
                .sum::<IntType>()
        })
        .collect()
}

pub fn step1(s: &str) -> Answer {
    distances(s).into_iter().min().unwrap().into()
}

pub fn step2(s: &str) -> Answer {
    distances(s).into_iter().max().unwrap().into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;

    #[test]
    fn step1_finds_correct_answer() {
        assert_eq!(step1(INPUT), Answer::Unsigned(605));
    }
}
