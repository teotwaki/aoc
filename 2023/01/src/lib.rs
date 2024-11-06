use common::Answer;

fn chars_to_number(first: char, second: char) -> u32 {
    first.to_digit(10).unwrap() * 10 + second.to_digit(10).unwrap()
}

fn clean_input(s: &str) -> String {
    s.replace("zero", "z0o")
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

pub fn step1(s: &str) -> Answer {
    s.lines()
        .map(|l| {
            (
                l.chars().find(|c| c.is_numeric()).unwrap(),
                l.chars().rev().find(|c| c.is_numeric()).unwrap(),
            )
        })
        .map(|(c1, c2)| chars_to_number(c1, c2))
        .sum::<u32>()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let s = clean_input(s);

    step1(&s)
}
