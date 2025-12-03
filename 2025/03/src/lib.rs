use common::Answer;

type IntType = u64;

fn parse(s: &str) -> Vec<Vec<IntType>> {
    s.lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as IntType)
                .collect()
        })
        .collect()
}

fn max_to_nine(batteries: &[IntType]) -> (usize, IntType) {
    let mut idx = 0;
    let mut max = 0;

    for (i, b) in batteries.iter().enumerate() {
        if *b > max {
            idx = i;
            max = *b;

            if max == 9 {
                break;
            }
        }
    }

    (idx, max)
}

fn largest_joltage(batteries: &[IntType], battery_count: usize) -> IntType {
    if battery_count == 1 {
        let (_, joltage) = max_to_nine(batteries);

        joltage
    } else {
        let count = battery_count - 1;
        let (idx, joltage) = max_to_nine(&batteries[..batteries.len() - count]);
        let joltage = joltage * 10u64.pow(count as u32);

        joltage + largest_joltage(&batteries[idx + 1..], count)
    }
}

fn calculate_joltage(s: &str, batteries: usize) -> Answer {
    parse(s)
        .iter()
        .map(|b| largest_joltage(b, batteries))
        .sum::<IntType>()
        .into()
}

pub fn step1(s: &str) -> Answer {
    calculate_joltage(s, 2)
}

pub fn step2(s: &str) -> Answer {
    calculate_joltage(s, 12)
}

#[cfg(test)]
mod test_2025_03 {
    use super::*;

    const INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn step1_finds_correct_sample_value() {
        assert_eq!(step1(INPUT), Answer::Unsigned(357))
    }
}
