use common::{utils::number_length, Answer};
use num::Integer;
use rustc_hash::FxHashMap;

type Stone = u64;

fn parse(s: &str) -> FxHashMap<Stone, usize> {
    s.split_whitespace()
        .map(|s| (s.parse::<Stone>().unwrap(), 1))
        .collect()
}

fn blink(stones: &mut FxHashMap<Stone, usize>) {
    stones
        .clone()
        .into_iter()
        .filter(|(_, v)| *v > 0)
        .for_each(|(stone, count)| {
            if stone == 0 {
                *stones.entry(1).or_default() += count;
            } else {
                let len = number_length(stone);

                if len % 2 == 0 {
                    let (left, right) = stone.div_rem(&10u64.pow((len / 2) as u32));
                    *stones.entry(left).or_default() += count;
                    *stones.entry(right).or_default() += count;
                } else {
                    *stones.entry(stone * 2024).or_default() += count;
                }
            }
            stones
                .entry(stone)
                .and_modify(|v| *v = v.saturating_sub(count));
        });
}

fn blink_n(s: &str, n: usize) -> usize {
    let mut stones = parse(s);

    (0..n).for_each(|_| {
        blink(&mut stones);
    });

    stones.values().sum::<usize>()
}

pub fn step1(s: &str) -> Answer {
    blink_n(s, 25).into()
}

pub fn step2(s: &str) -> Answer {
    blink_n(s, 75).into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blink_correctly_transforms_72() {
        let mut stones = FxHashMap::default();
        stones.insert(72, 1);

        blink(&mut stones);

        assert_eq!(stones.get(&72), Some(&0));
        assert_eq!(stones.get(&7), Some(&1));
        assert_eq!(stones.get(&2), Some(&1));
    }

    #[test]
    fn blink_finds_correct_values() {
        // initial arrangement
        let mut stones = FxHashMap::default();
        stones.insert(125, 1);
        stones.insert(17, 1);

        assert_eq!(stones.len(), 2);

        blink(&mut stones);

        // after 1 blink
        assert_eq!(stones.values().filter(|v| **v != 0).count(), 3);
        assert_eq!(stones.get(&253000), Some(&1));
        assert_eq!(stones.get(&1), Some(&1));
        assert_eq!(stones.get(&7), Some(&1));

        blink(&mut stones);

        // after 2 blinks
        assert_eq!(stones.values().filter(|v| **v != 0).count(), 4);
        assert_eq!(stones.get(&253), Some(&1));
        assert_eq!(stones.get(&0), Some(&1));
        assert_eq!(stones.get(&2024), Some(&1));
        assert_eq!(stones.get(&14168), Some(&1));

        blink(&mut stones);

        // after 3 blinks
        assert_eq!(stones.values().filter(|v| **v != 0).count(), 5);
        assert_eq!(stones.get(&512072), Some(&1));
        assert_eq!(stones.get(&1), Some(&1));
        assert_eq!(stones.get(&20), Some(&1));
        assert_eq!(stones.get(&24), Some(&1));
        assert_eq!(stones.get(&28676032), Some(&1));

        blink(&mut stones);

        // after 4 blinks
        assert_eq!(stones.values().filter(|v| **v != 0).count(), 8);
        assert_eq!(stones.get(&512), Some(&1));
        assert_eq!(stones.get(&72), Some(&1));
        assert_eq!(stones.get(&2024), Some(&1));
        assert_eq!(stones.get(&2), Some(&2));
        assert_eq!(stones.get(&0), Some(&1));
        assert_eq!(stones.get(&4), Some(&1));
        assert_eq!(stones.get(&2867), Some(&1));
        assert_eq!(stones.get(&6032), Some(&1));

        blink(&mut stones);

        // after 5 blinks
        assert_eq!(stones.values().filter(|v| **v != 0).count(), 12);
        assert_eq!(stones.get(&1036288), Some(&1));
        assert_eq!(stones.get(&7), Some(&1));
        assert_eq!(stones.get(&2), Some(&1));
        assert_eq!(stones.get(&20), Some(&1));
        assert_eq!(stones.get(&24), Some(&1));
        assert_eq!(stones.get(&4048), Some(&2));
        assert_eq!(stones.get(&1), Some(&1));
        assert_eq!(stones.get(&8096), Some(&1));
        assert_eq!(stones.get(&28), Some(&1));
        assert_eq!(stones.get(&67), Some(&1));
        assert_eq!(stones.get(&60), Some(&1));
        assert_eq!(stones.get(&32), Some(&1));
    }
}
