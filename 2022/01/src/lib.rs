use common::Answer;

fn get_sums(s: &str) -> Vec<i32> {
    let mut sums: Vec<i32> = s
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    sums.sort_unstable();

    sums
}

pub fn step1(s: &str) -> Answer {
    let max = *get_sums(s).iter().max().unwrap();

    max.into()
}

pub fn step2(s: &str) -> Answer {
    get_sums(s).iter().rev().take(3).sum::<i32>().into()
}
