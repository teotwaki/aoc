use common::Answer;
use itertools::Itertools;

type IntType = u16;

fn parse(s: &str) -> Vec<IntType> {
    s.lines().map(|l| l.parse::<IntType>().unwrap()).collect()
}

pub fn step1(s: &str) -> Answer {
    let containers = parse(s);

    containers
        .iter()
        .powerset()
        .filter(|perm| perm.iter().copied().sum::<IntType>() == 150)
        .count()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let containers = parse(s);

    let sets = containers
        .iter()
        .powerset()
        .filter(|perm| perm.iter().copied().sum::<IntType>() == 150)
        .collect::<Vec<_>>();

    let min_containers = sets.iter().map(|c| c.len()).min().unwrap();

    sets.iter()
        .filter(|c| c.len() == min_containers)
        .count()
        .into()
}
