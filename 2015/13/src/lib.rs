use itertools::Itertools;
use std::collections::HashMap;

use common::Answer;

type IntType = i32;
type Mapping<'a> = HashMap<&'a str, HashMap<&'a str, IntType>>;

fn parse(s: &str) -> Mapping {
    let mut mapping: Mapping = HashMap::new();

    s.lines().for_each(|l| {
        let mut parts = l[..l.len() - 1].split_whitespace();

        let a = parts.next().unwrap();
        let action = match parts.nth(1).unwrap() {
            "gain" => 1,
            _ => -1,
        };
        let happiness = action * parts.next().unwrap().parse::<IntType>().unwrap();
        let b = parts.nth(6).unwrap();

        mapping.entry(a).or_default().insert(b, happiness);
    });

    mapping
}

fn calculate_happiness(mapping: &Mapping) -> IntType {
    let happiness = |a: &str, b: &str| -> IntType { mapping[a][b] + mapping[b][a] };

    mapping
        .keys()
        .permutations(mapping.len())
        .map(|seats| {
            seats
                .windows(2)
                .map(|w| happiness(w[0], w[1]))
                .sum::<IntType>()
                + happiness(seats[0], seats[seats.len() - 1])
        })
        .max()
        .unwrap()
}

pub fn step1(s: &str) -> Answer {
    let mapping = parse(s);

    calculate_happiness(&mapping).into()
}

pub fn step2(s: &str) -> Answer {
    let mut mapping = parse(s);

    let guests = mapping.keys().copied().collect::<Vec<_>>();

    guests.iter().for_each(|k| {
        mapping.entry("me").or_default().insert(k, 0);
        mapping.entry(k).or_default().insert("me", 0);
    });

    calculate_happiness(&mapping).into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_happiness_finds_correct_result() {
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#;

        assert_eq!(calculate_happiness(&parse(input)), 330);
    }

    #[test]
    fn calculate_happiness_finds_correct_result_with_me() {
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Alice would gain 0 happiness units by sitting next to me.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Bob would gain 0 happiness units by sitting next to me.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
Carol would gain 0 happiness units by sitting next to me.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
David would gain 0 happiness units by sitting next to me.
me would gain 0 happiness units by sitting next to Alice.
me would gain 0 happiness units by sitting next to Bob.
me would gain 0 happiness units by sitting next to Carol.
me would gain 0 happiness units by sitting next to David."#;

        assert_eq!(calculate_happiness(&parse(input)), 286);
    }
}
