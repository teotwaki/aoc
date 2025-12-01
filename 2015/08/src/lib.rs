use common::Answer;

pub fn step1(s: &str) -> Answer {
    let apostrophes = s.lines().count() * 2;
    let s = s.replace("\n", "");
    let code_length = s.len();
    let mut s = s.replace(r"\\", "B").replace(r#"\""#, "A");

    while let Some(pos) = s.find(r"\x") {
        s.replace_range(pos..(pos + 4), "X");
    }

    let data_length = s.len() - apostrophes;

    (code_length - data_length).into()
}

pub fn step2(s: &str) -> Answer {
    let apostrophes = s.lines().count() * 2;
    let s = s.replace("\n", "");
    let code_length = s.len();

    let s = s.replace(r"\", r"\\").replace(r#"""#, r#"\""#);

    (s.len() + apostrophes - code_length).into()
}

#[cfg(test)]
mod test_2015_08 {
    use super::*;

    const INPUT: &str = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

    #[test]
    fn step1_finds_correct_answer() {
        assert_eq!(step1(INPUT), Answer::Unsigned(12));
    }

    #[test]
    fn step2_finds_correct_answer() {
        assert_eq!(step2(INPUT), Answer::Unsigned(19));
    }
}
