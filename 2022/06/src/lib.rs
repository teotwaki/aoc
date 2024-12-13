use common::Answer;
use rustc_hash::FxHashSet;
use std::str::Chars;

fn has_unique_elements(chars: Chars) -> bool {
    let mut uniq = FxHashSet::default();
    chars.into_iter().all(|x| uniq.insert(x))
}

fn find_marker(s: &str, length: usize) -> usize {
    for i in 0..(s.len() - length) {
        if has_unique_elements(s[i..(i + length)].chars()) {
            return i + length;
        }
    }

    unreachable!()
}

pub fn step1(s: &str) -> Answer {
    let line = *s.lines().collect::<Vec<_>>().first().unwrap();

    find_marker(line, 4).into()
}

pub fn step2(s: &str) -> Answer {
    let line = *s.lines().collect::<Vec<_>>().first().unwrap();

    find_marker(line, 14).into()
}
