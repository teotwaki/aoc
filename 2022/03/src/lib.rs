use common::Answer;

fn priority(c: char) -> i32 {
    let ascii: i32 = c as i32;

    if ascii >= 'a' as i32 {
        ascii - 'a' as i32 + 1
    } else {
        ascii - 'A' as i32 + 27
    }
}

fn find_common(group: &[&str]) -> i32 {
    group
        .iter()
        .map(|l| split_halves(l))
        .map(|(left, right)| {
            for c in left.chars() {
                if right.contains(c) {
                    return priority(c);
                }
            }
            unreachable!()
        })
        .sum()
}

fn find_badge(group: &[&str]) -> char {
    for c in group[0].chars() {
        if group[1].contains(c) && group[2].contains(c) {
            return c;
        }
    }
    unreachable!()
}

fn split_halves(l: &str) -> (String, String) {
    (
        l[0..(l.len() / 2)].to_string(),
        l[(l.len() / 2)..l.len()].to_string(),
    )
}

pub fn step1(s: &str) -> Answer {
    let lines: Vec<&str> = s.lines().collect();

    lines.chunks(3).map(find_common).sum::<i32>().into()
}

pub fn step2(s: &str) -> Answer {
    let lines: Vec<&str> = s.lines().collect();

    lines
        .chunks(3)
        .map(|group| priority(find_badge(group)))
        .sum::<i32>()
        .into()
}
