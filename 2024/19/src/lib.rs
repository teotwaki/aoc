use common::Answer;
use rustc_hash::FxHashMap;

fn parse(s: &str) -> (Vec<&str>, Vec<&str>) {
    let mut parts = s.split("\n\n");

    let mut patterns = parts
        .next()
        .unwrap()
        .split_whitespace()
        .flat_map(|s| s.split(','))
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    patterns.sort_by_key(|s| !s.len());

    let designs = parts.next().unwrap().lines().collect();

    (patterns, designs)
}

fn count_possible<'a, 'b>(
    design: &'b str,
    patterns: &'a Vec<&'b str>,
    cache: &'a mut FxHashMap<&'b str, usize>,
) -> usize {
    if let Some(value) = cache.get(design) {
        *value
    } else {
        let sum = patterns
            .iter()
            .filter(|&pattern| design.starts_with(pattern))
            .map(|&pattern| count_possible(&design[pattern.len()..], patterns, cache))
            .sum();

        cache.insert(design, sum);

        sum
    }
}

fn possible_designs(s: &str) -> Vec<usize> {
    let (patterns, designs) = parse(s);
    let mut cache = FxHashMap::default();

    cache.insert("", 1);

    designs
        .into_iter()
        .map(|design| count_possible(design, &patterns, &mut cache))
        .collect()
}

pub fn step1(s: &str) -> Answer {
    possible_designs(s)
        .into_iter()
        .filter(|value| *value != 0)
        .count()
        .into()
}

pub fn step2(s: &str) -> Answer {
    possible_designs(s).iter().sum::<usize>().into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn step1_example_correct_answer() {
        assert_eq!(step1(INPUT), Answer::Unsigned(6));
    }

    #[test]
    fn step2_example_correct_answer() {
        assert_eq!(step2(INPUT), Answer::Unsigned(16));
    }
}
