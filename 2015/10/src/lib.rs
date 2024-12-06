use common::Answer;
use std::fmt::Write;

#[derive(Debug, Clone, Copy, Default)]
struct Digit {
    value: u8,
    count: u8,
}

fn look_and_say(s: &str) -> String {
    let mut digits = vec![];
    let mut digit = Digit::default();

    s.chars().for_each(|c| {
        let i = c.to_digit(10).unwrap() as u8;

        if digit.value == i {
            digit.count += 1;
            return;
        } else if digit.count != 0 {
            digits.push(digit);
        }

        digit.value = i;
        digit.count = 1;
    });

    digits.push(digit);

    let mut s = String::new();

    digits
        .iter()
        .for_each(|d| write!(s, "{}{}", d.count, d.value).unwrap());

    s
}

fn look_n_say(s: &str, n: usize) -> usize {
    let mut s = look_and_say(s.trim_end());

    for _ in 1..n {
        s = look_and_say(&s);
    }

    s.len()
}

pub fn step1(s: &str) -> Answer {
    look_n_say(s, 40).into()
}

pub fn step2(s: &str) -> Answer {
    look_n_say(s, 50).into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"1"#;

    #[test]
    fn look_and_say_behaves_as_expected() {
        let mut s = INPUT.to_string();

        for _ in 0..5 {
            s = look_and_say(&s);
        }

        assert_eq!(s, "312211");
    }
}
