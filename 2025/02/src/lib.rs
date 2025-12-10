use common::{
    Answer,
    utils::{factors, number_length},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type IntType = u64;

#[derive(Debug, Clone, Copy)]
struct Range(IntType, IntType);

impl Range {
    pub fn new(start: IntType, end: IntType) -> Self {
        Self(start, end)
    }
}

fn parse(s: &str) -> Vec<Range> {
    s.trim_end()
        .split(',')
        .map(|s| {
            s.split_once('-')
                .and_then(|(start, end)| {
                    let start = start.parse().ok()?;
                    let end = end.parse().ok()?;
                    Some(Range::new(start, end))
                })
                .unwrap()
        })
        .collect()
}

fn is_valid_id(id: IntType) -> bool {
    let length = number_length(id);

    if length.is_multiple_of(2) {
        let offset = 10u64.pow((length / 2) as u32);
        let a = id / offset;
        let b = id % offset;

        a != b
    } else {
        true
    }
}

fn split_digits(n: IntType, parts: usize) -> Vec<IntType> {
    let factor = 10u64.pow((number_length(n) / parts) as u32);
    let mut result = Vec::new();
    let mut current = n;

    for _ in 0..parts {
        let digit = current % factor;
        result.push(digit);
        current /= factor;
    }

    result
}

fn is_valid_complex_id(id: IntType) -> bool {
    let length = number_length(id);
    let factors = factors(length as u64);

    factors.iter().skip(1).all(|&factor| {
        let segments = split_digits(id, factor as usize);
        segments.iter().skip(1).any(|&v| v != segments[0])
    })
}

fn run(s: &str, validation_func: fn(IntType) -> bool) -> Answer {
    Answer::Unsigned(
        parse(s)
            .par_iter()
            .flat_map(|r| r.0..=r.1)
            .filter(|id| !validation_func(*id))
            .sum(),
    )
}

pub fn step1(s: &str) -> Answer {
    run(s, is_valid_id)
}

pub fn step2(s: &str) -> Answer {
    run(s, is_valid_complex_id)
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    const INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn parse_extracts_correct_number_of_ranges() {
        assert_eq!(parse(INPUT).len(), 11);
    }

    #[test]
    fn step1_correctly_counts_invalid_ids() {
        assert_eq!(step1(INPUT), Answer::Unsigned(1227775554));
    }

    #[test]
    fn step2_correctly_counts_invalid_ids() {
        assert_eq!(step2(INPUT), Answer::Unsigned(4174379265));
    }

    #[parameterized(input = { 123, 124, 1244, 332994 })]
    fn is_valid_id_identifies_correct_values(input: IntType) {
        assert!(is_valid_id(input));
    }

    #[parameterized(input = { 1212, 22, 123123 })]
    fn is_valid_id_identifies_incorrect_values(input: IntType) {
        assert!(!is_valid_id(input));
    }

    #[parameterized(input = { 12, 13, 998, 2121212118 })]
    fn is_valid_complex_id_identifies_correct_values(input: IntType) {
        assert!(is_valid_complex_id(input));
    }

    #[parameterized(
        input = { 11, 22, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656, 824824824, 2121212121 }
    )]
    fn is_valid_complex_id_identifies_incorrect_values(input: IntType) {
        assert!(!is_valid_complex_id(input));
    }
}
