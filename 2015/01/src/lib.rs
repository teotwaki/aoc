use common::Answer;

fn parse(s: &str) -> (usize, usize) {
    (
        s.chars().filter(|&c| c == '(').count(),
        s.chars().filter(|&c| c == ')').count(),
    )
}

pub fn step1(s: &str) -> Answer {
    let (up, down) = parse(s);

    (up as i32 - down as i32).into()
}

pub fn step2(s: &str) -> Answer {
    let mut floor = 0;

    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }

        if floor == -1 {
            return (i + 1).into();
        }
    }

    ().into()
}
