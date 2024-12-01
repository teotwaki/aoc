use common::Answer;

type IntType = u32;

fn parse(s: &str) -> (Vec<IntType>, Vec<IntType>) {
    s.lines()
        .map(|s| s.split("   "))
        .map(|mut parts| (parts.next().unwrap(), parts.next().unwrap()))
        .map(|(a, b)| (a.parse::<IntType>().unwrap(), b.parse::<IntType>().unwrap()))
        .unzip()
}

pub fn step1(s: &str) -> Answer {
    let (mut a, mut b) = parse(s);

    a.sort();
    b.sort();

    a.iter()
        .zip(b.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum::<IntType>()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let (a, b) = parse(s);

    a.iter()
        .map(|&a| a as usize * b.iter().filter(|&&b| a == b).count())
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn parse_finds_correct_number_of_lines() {
        assert_eq!(parse(INPUT).0.len(), 6);
    }

    #[test]
    fn parse_finds_correct_first_and_last_items() {
        let data = parse(INPUT);

        assert_eq!(data.0.first().unwrap(), &3);
        assert_eq!(data.1.first().unwrap(), &4);

        assert_eq!(data.0.last().unwrap(), &3);
        assert_eq!(data.1.last().unwrap(), &3);
    }

    #[test]
    fn step2_finds_right_answer() {
        let answer = step2(INPUT);

        assert_eq!(answer, Answer::Unsigned(31))
    }
}
