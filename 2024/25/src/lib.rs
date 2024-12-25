use common::Answer;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
enum KeyLock {
    Key([u8; 5]),
    Lock([u8; 5]),
}

impl KeyLock {
    fn unlocks(&self, other: &Self) -> bool {
        use KeyLock::*;

        match (self, other) {
            (Lock(_), _) | (_, Key(_)) => false,
            (Key(key_pins), Lock(lock_pins)) => {
                key_pins.iter().zip(lock_pins).all(|(a, b)| a + b <= 5)
            }
        }
    }
}

impl From<&str> for KeyLock {
    fn from(value: &str) -> Self {
        let mut pins = [0; 5];

        if value.starts_with("#####") {
            value.lines().skip(1).for_each(|line| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .for_each(|(i, _)| pins[i] += 1)
            });

            KeyLock::Lock(pins)
        } else {
            value.lines().rev().skip(1).for_each(|line| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .for_each(|(i, _)| pins[i] += 1)
            });

            KeyLock::Key(pins)
        }
    }
}

fn parse(s: &str) -> Vec<KeyLock> {
    s.split("\n\n").map(KeyLock::from).collect()
}

pub fn step1(s: &str) -> Answer {
    let keylocks = parse(s);
    let keys = keylocks
        .iter()
        .filter(|x| matches!(x, KeyLock::Key(_)))
        .collect::<Vec<_>>();

    let locks = keylocks
        .iter()
        .filter(|x| matches!(x, KeyLock::Lock(_)))
        .collect::<Vec<_>>();

    keys.par_iter()
        .map(|key| locks.iter().filter(|lock| key.unlocks(lock)).count())
        .sum::<usize>()
        .into()
}

pub fn step2(_: &str) -> Answer {
    ().into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

    #[test]
    fn step1_correctly_solves_first_example() {
        assert_eq!(step1(INPUT), Answer::Unsigned(3));
    }
}
