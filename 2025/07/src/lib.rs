use common::Answer;
use rustc_hash::{FxHashMap, FxHashSet};

type IntType = u8;

fn parse(s: &str) -> Vec<Vec<IntType>> {
    s.lines()
        .map(|l| {
            l.char_indices()
                .filter(|(_, c)| *c == '^' || *c == 'S')
                .map(|(x, _)| x as IntType)
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn step1(s: &str) -> Answer {
    let splitters = parse(s);
    let mut rays = FxHashSet::default();
    let mut splits = 0u32;

    rays.insert(splitters[0][0]);

    splitters.iter().skip(1).for_each(|splitters| {
        splitters.iter().for_each(|splitter| {
            if rays.contains(splitter) {
                splits += 1;
                rays.remove(splitter);

                rays.insert(splitter - 1);
                rays.insert(splitter + 1);
            }
        });
    });

    splits.into()
}

fn dfs(
    row: usize,
    x: IntType,
    splitters: &[Vec<IntType>],
    cache: &mut FxHashMap<(usize, IntType), u64>,
) -> u64 {
    if let Some(&cached) = cache.get(&(row, x)) {
        return cached;
    }

    if row >= splitters.len() {
        return 1;
    }

    let res = if splitters[row].contains(&x) {
        let left = dfs(row + 1, x - 1, splitters, cache);
        let right = dfs(row + 1, x + 1, splitters, cache);

        left + right
    } else {
        dfs(row + 1, x, splitters, cache)
    };

    cache.insert((row, x), res);

    res
}

pub fn step2(s: &str) -> Answer {
    let splitters = parse(s);
    let mut cache = FxHashMap::default();
    let timelines = dfs(1, splitters[0][0], &splitters, &mut cache);

    timelines.into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn step1_finds_correct_sample_answer() {
        assert_eq!(step1(INPUT), Answer::Unsigned(21));
    }

    #[test]
    fn step2_finds_correct_sample_answer() {
        assert_eq!(step2(INPUT), Answer::Unsigned(40));
    }
}
