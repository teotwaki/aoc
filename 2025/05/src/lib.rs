use common::Answer;

type IntType = u64;

#[derive(Debug, Clone, Copy)]
struct Range(IntType, IntType);

impl Range {
    pub fn new(start: IntType, end: IntType) -> Self {
        Self(start, end)
    }

    pub fn contains(&self, needle: IntType) -> bool {
        self.0 <= needle && self.1 >= needle
    }
}

fn parse(s: &str) -> (Vec<Range>, Vec<IntType>) {
    let mut parts = s.split("\n\n");

    let ranges = parts
        .next()
        .unwrap()
        .lines()
        .map(|s| {
            s.split_once('-')
                .map(|(s, e)| Range::new(s.parse().unwrap(), e.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let values = parts
        .next()
        .unwrap()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    (ranges, values)
}

pub fn step1(s: &str) -> Answer {
    let (database, ids) = parse(s);

    ids.iter()
        .filter(|&&id| database.iter().any(|&range| range.contains(id)))
        .count()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let (ranges, _) = parse(s);
    let mut processed: Vec<Range> = vec![];

    ranges.iter().for_each(|r| {
        println!("Processing {}-{}", r.0, r.1);
        let mut add = true;

        processed.iter_mut().for_each(|rp| {
            println!("Comparing against {}-{}", rp.0, rp.1);

            if rp.contains(r.0) && !rp.contains(r.1) {
                rp.1 = r.1;
                add = false;
            } else if rp.contains(r.1) && !rp.contains(r.0) {
                rp.0 = r.0;
                add = false;
            } else if rp.contains(r.1) && rp.contains(r.0) {
                println!("No overlap, adding outright");
                add = false;
            }
        });

        if add {
            processed.push(*r);
        }
    });

    dbg!(processed)
        .iter()
        .map(|r| r.1 - r.0 + 1)
        .sum::<IntType>()
        .into()
}

#[cfg(test)]
mod test_2025_05 {
    use super::*;

    const INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn step1_finds_correct_sample_value() {
        assert_eq!(step1(INPUT), Answer::Unsigned(3));
    }

    #[test]
    fn step2_finds_correct_sample_value() {
        assert_eq!(step2(INPUT), Answer::Unsigned(14));
    }
}
