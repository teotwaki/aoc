use common::Answer;
use itertools::Itertools;

fn is_nice(s: &str) -> bool {
    static VOWELS: &str = "aeiou";
    static BANNED: [&str; 4] = ["ab", "cd", "pq", "xy"];

    s.chars().filter(|&c| VOWELS.contains(c)).count() >= 3
        && s.chars().tuple_windows().any(|(a, b)| a == b)
        && BANNED.iter().all(|banned| !s.contains(banned))
}

pub fn step1(s: &str) -> Answer {
    s.lines().filter(|l| is_nice(l)).count().into()
}

fn is_really_nice(s: &str) -> bool {
    s.chars()
        .tuple_windows::<(_, _, _)>()
        .any(|(a, _, b)| a == b)
        && (0..(s.len() - 1)).any(|i| s[i + 2..].contains(&s[i..=i + 1]))
}

pub fn step2(s: &str) -> Answer {
    s.lines().filter(|l| is_really_nice(l)).count().into()
}

#[cfg(test)]
mod test_2015_05 {
    use super::*;

    #[test]
    fn is_really_nice_handles_test_data() {
        assert!(is_really_nice("qjhvhtzxzqqjkmpb"));
        assert!(is_really_nice("xxyxx"));
        assert!(!is_really_nice("uurcxstgmygtbstg"));
        assert!(!is_really_nice("ieodomkazucvgmuy"));
    }
}
