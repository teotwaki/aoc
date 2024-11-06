use common::Answer;

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn outcomes(&self) -> Vec<u64> {
        (0..=self.time)
            .map(|speed| (self.time - speed) * speed)
            .collect()
    }
}

fn step1_races() -> Vec<Race> {
    vec![
        Race {
            time: 44,
            distance: 277,
        },
        Race {
            time: 89,
            distance: 1136,
        },
        Race {
            time: 96,
            distance: 1890,
        },
        Race {
            time: 91,
            distance: 1768,
        },
    ]
}

fn step2_races() -> Vec<Race> {
    vec![Race {
        time: 44899691,
        distance: 277113618901768,
    }]
}

pub fn step1(_: &str) -> Answer {
    let races = step1_races();

    races
        .iter()
        .map(|r| r.outcomes().iter().filter(|o| **o > r.distance).count())
        .product::<usize>()
        .into()
}

pub fn step2(_: &str) -> Answer {
    let races = step2_races();

    races
        .iter()
        .map(|r| r.outcomes().iter().filter(|o| **o > r.distance).count())
        .product::<usize>()
        .into()
}
