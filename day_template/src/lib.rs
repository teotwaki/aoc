use common::Answer;

type IntType = u16;

fn parse(s: &str) -> Vec<IntType> {
    s.lines().map(|l| l.parse::<IntType>().unwrap()).collect()
}

pub fn step1(s: &str) -> Answer {
    unimplemented!()
}

pub fn step2(_: &str) -> Answer {
    ().into()
}

#[cfg(test)]
mod test_2025_08 {
    use super::*;

    const INPUT: &str = r#""#;

    #[test]
    fn parse_extracts_correct_number_of_lines() {
        assert_eq!(parse(INPUT).len(), 10);
    }

    #[test]
    fn step1_computes_expected_sample_result() {
        assert_eq!(step1(INPUT), Answer::Unsigned(5));
    }

    /*
    #[test]
    fn step2_computes_expected_sample_result() {
        assert_eq!(step2(INPUT), Answer::Unsigned(5));
    }
    */
}
