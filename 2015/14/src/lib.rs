use common::Answer;

type IntType = u16;

#[derive(Debug, Clone, Copy)]
struct Reindeer {
    speed: IntType,
    fly_duration: IntType,
    rest_duration: IntType,
}

fn parse(s: &str) -> impl Iterator<Item = Reindeer> + '_ {
    s.lines().map(|l| {
        let mut parts = l.split_whitespace();

        let speed = parts.nth(3).and_then(|s| s.parse().ok()).unwrap();
        let fly_duration = parts.nth(2).and_then(|s| s.parse().ok()).unwrap();
        let rest_duration = parts.nth(6).and_then(|s| s.parse().ok()).unwrap();

        Reindeer {
            speed,
            fly_duration,
            rest_duration,
        }
    })
}

fn calculate_distance(r: Reindeer, race_duration: IntType) -> IntType {
    let cycle_duration = r.fly_duration + r.rest_duration;
    let cycles = race_duration / cycle_duration;
    let remainder = (race_duration - (cycles * cycle_duration)).min(r.fly_duration);

    r.speed * r.fly_duration * cycles + remainder * r.speed
}

pub fn step1(s: &str) -> Answer {
    parse(s)
        .map(|r| calculate_distance(r, 2503))
        .max()
        .unwrap()
        .into()
}

#[derive(Debug, Clone, Copy)]
struct ReindeerSimulator {
    reindeer: Reindeer,
    seconds: IntType,
    score: IntType,
}

impl ReindeerSimulator {
    fn tick(&mut self) -> IntType {
        self.seconds += 1;

        let cycle_duration = self.reindeer.fly_duration + self.reindeer.rest_duration;
        let cycles = self.seconds / cycle_duration;
        let remainder = (self.seconds - (cycles * cycle_duration)).min(self.reindeer.fly_duration);

        self.reindeer.speed * self.reindeer.fly_duration * cycles + remainder * self.reindeer.speed
    }
}

pub fn step2(s: &str) -> Answer {
    let mut simulators = parse(s)
        .map(|r| ReindeerSimulator {
            reindeer: r,
            seconds: 0,
            score: 0,
        })
        .collect::<Vec<_>>();

    for _ in 0..2503 {
        let distances = simulators.iter_mut().map(|s| s.tick()).collect::<Vec<_>>();
        let max = distances.iter().max().unwrap();
        distances
            .iter()
            .enumerate()
            .filter(|&(_, score)| score == max)
            .for_each(|(i, _)| simulators[i].score += 1);
    }

    simulators.iter().map(|s| s.score).max().unwrap().into()
}

#[cfg(test)]
mod test_2015_14 {
    use super::*;

    #[test]
    fn calculate_distance_calculates_proper_distances() {
        let comet = Reindeer {
            speed: 14,
            fly_duration: 10,
            rest_duration: 127,
        };

        assert_eq!(calculate_distance(comet, 1000), 1120);
    }
}
