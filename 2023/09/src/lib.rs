use common::Answer;

fn calculate_diffs(s: &str) -> Vec<Vec<Vec<i32>>> {
    s.lines()
        .map(|l| {
            l.split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|hist| {
            let mut diffs = vec![hist];

            loop {
                diffs.push(
                    diffs
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect(),
                );

                if diffs.last().unwrap().iter().all(|x| *x == 0) {
                    break;
                }
            }

            diffs
        })
        .collect()
}

pub fn step1(s: &str) -> Answer {
    let diffs = calculate_diffs(s);

    diffs
        .iter()
        .map(|d| d.iter().map(|d| *d.last().unwrap()).sum::<i32>())
        .sum::<i32>()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let diffs = calculate_diffs(s);

    diffs
        .iter()
        .map(|d| {
            d.iter()
                .map(|d| *d.first().unwrap())
                .rev()
                .fold(0, |acc, x| x - acc)
        })
        .sum::<i32>()
        .into()
}
