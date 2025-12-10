use common::Answer;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
struct Letter(u8);

impl From<char> for Letter {
    fn from(value: char) -> Self {
        Letter(value as u8)
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

impl Letter {
    fn increment(&mut self) -> bool {
        match self.0 {
            122 => {
                self.0 = 97;
                false
            }
            104 | 107 | 110 => {
                self.0 += 2;
                true
            }
            _ => {
                self.0 += 1;
                true
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Password(Vec<Letter>);

impl From<&str> for Password {
    fn from(value: &str) -> Self {
        Password(value.trim_end().chars().map(Letter::from).collect())
    }
}

fn repetitions(arr: &[u8]) -> usize {
    let mut prev = 0;
    let mut count = 0;

    for &c in arr.iter() {
        if c == prev {
            count += 1;
            prev = 0;
        } else {
            prev = c;
        }
    }

    count
}

impl Password {
    fn increment(&mut self) {
        for c in self.0.iter_mut().rev() {
            if c.increment() {
                break;
            }
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        self.0.iter().map(|c| c.0).collect()
    }

    fn is_valid(&self) -> bool {
        let vec = self.to_vec();

        vec.windows(3).any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]) && repetitions(&vec) >= 2
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self.0.iter() {
            l.fmt(f)?;
        }

        Ok(())
    }
}

pub fn step1(s: &str) -> Answer {
    let mut pw = Password::from(s);

    while !pw.is_valid() {
        pw.increment();
    }

    pw.to_string().into()
}

pub fn step2(s: &str) -> Answer {
    let mut pw = Password::from(s);

    while !pw.is_valid() {
        pw.increment();
    }

    pw.increment();

    while !pw.is_valid() {
        pw.increment();
    }

    pw.to_string().into()
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(
        input = { "hijklmmn", "abbceffg", "abbcegjk", "ghijklmn" },
    )]
    fn validate_can_invalidate_passwords(input: &str) {
        assert!(!Password::from(input).is_valid());
    }

    #[test]
    fn validate_can_validate_passwords() {
        assert!(Password::from("ghjaabcc").is_valid());
    }
}
