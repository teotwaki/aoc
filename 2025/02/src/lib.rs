use common::{Answer, utils::number_length};

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

pub fn step1(s: &str) -> Answer {
    Answer::Unsigned(
        parse(s)
            .iter()
            .flat_map(|r| r.0..=r.1)
            .filter(|id| !is_valid_id(*id))
            .sum(),
    )
}

pub fn step2(_: &str) -> Answer {
    ().into()
}

#[cfg(test)]
mod test_2025_02 {
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
}
