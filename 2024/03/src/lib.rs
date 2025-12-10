use common::Answer;
use regex::Regex;
use std::sync::LazyLock;

type IntType = u32;

fn parse(s: &str) -> impl Iterator<Item = (IntType, IntType)> + '_ {
    static EXPRESSION: &str = r"mul\((\d{1,3}),(\d{1,3})\)";
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(EXPRESSION).unwrap());

    RE.captures_iter(s).map(|caps| {
        let (_, [a, b]) = caps.extract();
        (a.parse::<IntType>().unwrap(), b.parse::<IntType>().unwrap())
    })
}

pub fn step1(s: &str) -> Answer {
    parse(s).map(|(a, b)| a * b).sum::<IntType>().into()
}

fn remove_instructions(s: &str) -> String {
    let mut data = s.to_owned();

    while let Some(start) = data.find("don't()") {
        let end = if let Some(end) = data[start..].find("do()") {
            end + start + 4
        } else {
            data.len()
        };

        data.replace_range(start..end, "");
    }

    data
}

pub fn step2(s: &str) -> Answer {
    step1(&remove_instructions(s))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

    #[test]
    fn parse_extracts_correct_number_of_items() {
        assert_eq!(parse(INPUT).count(), 4);
    }

    #[test]
    fn step1_finds_correct_anwer() {
        assert_eq!(step1(INPUT), Answer::Unsigned(161));
    }

    const EXAMPLE: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn step2_finds_correct_answer() {
        assert_eq!(step2(EXAMPLE), Answer::Unsigned(48));
    }
}
