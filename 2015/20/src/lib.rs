use common::Answer;

pub fn step1(_: &str) -> Answer {
    let n = 33100000;

    let max = n / 10;
    let mut houses = vec![0; max];

    for i in 1..max {
        for j in (i..max).step_by(i) {
            houses[j] += i as u32;
        }
    }

    houses
        .iter()
        .enumerate()
        .find(|(_, v)| (**v) as usize >= max)
        .map(|(k, _)| k)
        .unwrap()
        .into()
}

pub fn step2(_: &str) -> Answer {
    let n = 33100000;

    let max = n / 11;
    let mut houses = vec![0; max / 3];

    for i in 1..houses.len() {
        let mut count = 0;

        for j in (i..houses.len()).step_by(i) {
            houses[j] += i as u32;
            count += 1;

            if count == 50 {
                break;
            }
        }
    }

    houses
        .iter()
        .enumerate()
        .find(|(_, v)| **v >= max as u32)
        .map(|(k, _)| k)
        .unwrap()
        .into()
}
