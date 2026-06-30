use common::Answer;
use itertools::Itertools;
use rustc_hash::FxHashMap;

type IntType = i8;

fn parse_groups(s: &str) -> Vec<IntType> {
    s.split(',').map(|s| s.parse().unwrap()).collect()
}

fn parse(s: &str) -> Vec<(&str, Vec<IntType>)> {
    s.lines()
        .map(|s| {
            let mut parts = s.split_whitespace();
            let springs_state = parts.next().unwrap();
            let groups = parse_groups(parts.next().unwrap());

            (springs_state, groups)
        })
        .collect()
}

fn walk(states: &str, groups: &[IntType], cache: &mut FxHashMap<(usize, usize), usize>) -> usize {
    if groups.is_empty() {
        return if states.contains('#') { 0 } else { 1 };
    }

    if states.is_empty() {
        return 0;
    }

    let key = (states.len(), groups.len());
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    let mut total = 0;

    if !states.starts_with('#') {
        total += walk(&states[1..], groups, cache);
    }

    let group_size = groups[0] as usize;

    if group_size <= states.len()
        && !states[..group_size].contains('.')
        && (group_size == states.len() || states.chars().nth(group_size).unwrap() != '#')
    {
        let rest = if group_size == states.len() {
            &states[group_size..]
        } else {
            &states[group_size + 1..]
        };

        total += walk(rest, &groups[1..], cache);
    }

    cache.insert(key, total);
    total
}

fn count_possibilities(states: &str, groups: &[IntType]) -> usize {
    let mut cache = FxHashMap::default();
    walk(states, groups, &mut cache)
}

pub fn step1(s: &str) -> Answer {
    parse(s)
        .iter()
        .map(|(states, groups)| count_possibilities(states, groups))
        .sum::<usize>()
        .into()
}

fn unfold(s: &str) -> String {
    s.lines()
        .map(|l| l.split_whitespace())
        .map(|mut parts| {
            let states = parts.next().unwrap();
            let groups = parts.next().unwrap();

            let states = &[states; 5];
            let groups = &[groups; 5];

            (states.join("?"), groups.join(","))
        })
        .map(|(s, g)| format!("{s} {g}"))
        .join("\n")
}

pub fn step2(s: &str) -> Answer {
    let input = unfold(s);
    step1(&input)
}

#[cfg(test)]
mod test_2023_12 {
    use super::*;
    use parameterized::parameterized;

    const INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn parse_extracts_correct_number_of_lines() {
        assert_eq!(parse(INPUT).len(), 6);
    }

    #[parameterized(
        state = { '?', '.', '#' },
        count = { 3, 1, 3 }
    )]
    fn parse_reads_correct_states(state: char, count: usize) {
        let lines = parse(INPUT);
        let states = &lines[0].0;

        let unknowns = states.chars().filter(|&c| c == state).count();

        assert_eq!(unknowns, count);
    }

    #[test]
    fn parse_reads_correct_groups() {
        let lines = parse(INPUT);
        let groups = &lines[0].1;

        assert_eq!(*groups, vec![1, 1, 3]);
    }

    #[test]
    fn step1_computes_expected_sample_result() {
        assert_eq!(step1(INPUT), Answer::Unsigned(21));
    }

    #[test]
    fn unfold_expands_properly() {
        assert_eq!(unfold("... 1"), "...?...?...?...?... 1,1,1,1,1");
    }

    #[test]
    fn step2_computes_expected_sample_result() {
        assert_eq!(step2(INPUT), Answer::Unsigned(525152));
    }
}
