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

    #[test]
    fn is_valid_id_identifies_correct_values() {
        assert!(is_valid_id(123));
        assert!(is_valid_id(124));
        assert!(is_valid_id(1244));
        assert!(is_valid_id(332994));
    }

    #[test]
    fn is_valid_id_identifies_incorrect_values() {
        assert!(!is_valid_id(1212));
        assert!(!is_valid_id(22));
        assert!(!is_valid_id(123123));
    }

    #[test]
    fn is_valid_complex_id_identifies_correct_values() {
        assert!(is_valid_complex_id(12));
        assert!(is_valid_complex_id(13));
        assert!(is_valid_complex_id(998));
        assert!(is_valid_complex_id(998));
        assert!(is_valid_complex_id(2121212118));
    }

    #[test]
    fn is_valid_complex_id_identifies_incorrect_values() {
        assert!(!is_valid_complex_id(11));
        assert!(!is_valid_complex_id(22));
        assert!(!is_valid_complex_id(999));
        assert!(!is_valid_complex_id(1010));
        assert!(!is_valid_complex_id(1188511885));
        assert!(!is_valid_complex_id(222222));
        assert!(!is_valid_complex_id(446446));
        assert!(!is_valid_complex_id(38593859));
        assert!(!is_valid_complex_id(565656));
        assert!(!is_valid_complex_id(824824824));
        assert!(!is_valid_complex_id(2121212121));
    }
}
