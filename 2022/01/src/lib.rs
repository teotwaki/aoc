use common::Answer;
use itertools::Itertools;

fn get_sums(s: &str) -> impl DoubleEndedIterator<Item = i32> {
    s.split("\n\n")
        .map(|s| {
            s.lines()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sorted_unstable()
}

pub fn step1(s: &str) -> Answer {
    get_sums(s).max().unwrap().into()
}

pub fn step2(s: &str) -> Answer {
    get_sums(s).rev().take(3).sum::<i32>().into()
}

#[cfg(test)]
mod test_2022_01 {
    use super::*;

    const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn step1_produces_correct_result_based_on_sample_input() {
        assert_eq!(step1(INPUT), Answer::Signed(24000));
    }

    #[test]
    fn step2_produces_correct_result_based_on_sample_input() {
        assert_eq!(step2(INPUT), Answer::Signed(45000));
    }
}
