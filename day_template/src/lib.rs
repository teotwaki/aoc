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
mod test {
    use super::*;

    const INPUT: &str = r#""#;

    #[test]
    fn parse_extracts_correct_number_of_lines() {
        assert_eq!(parse(INPUT).len(), 10);
    }
}
