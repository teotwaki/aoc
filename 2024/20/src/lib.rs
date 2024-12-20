use common::{distances::manhattan, Answer, BooleanGrid, Coordinates};
use rayon::prelude::*;
use rustc_hash::FxHashMap;

type IntType = i16;
type Coords = Coordinates<IntType>;

fn parse(s: &str) -> (FxHashMap<Coords, usize>, Coords) {
    let mut map = BooleanGrid::new();
    let mut start = (0, 0).into();
    let mut end = (0, 0).into();

    s.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '.' => map.mark((x, y).into()),
            'S' => {
                map.mark((x, y).into());
                start = (x, y).into();
            }
            'E' => {
                map.mark((x, y).into());
                end = (x, y).into();
            }
            _ => {}
        })
    });

    let path = map
        .bfs(start, end, true)
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<FxHashMap<_, _>>();

    (path, end)
}

fn extended_neighbors(pos: Coords) -> Vec<Coords> {
    vec![
        pos.up().up(),
        pos.right().right(),
        pos.down().down(),
        pos.left().left(),
    ]
}

pub fn step1(s: &str) -> Answer {
    let (path, end) = parse(s);

    path.par_iter()
        .filter(|(pos, _)| **pos != end)
        .map(|(pos, dist)| {
            extended_neighbors(*pos)
                .into_iter()
                .filter(|pos| path.get(pos).unwrap_or(&0).saturating_sub(dist + 2) >= 100)
                .count()
        })
        .sum::<usize>()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let (path, end) = parse(s);

    path.par_iter()
        .filter(|(pos, _)| **pos != end)
        .map(|(pos, dist)| {
            path.iter()
                .filter(|&(p, d)| {
                    let distance = manhattan(*pos, *p) as usize;
                    pos != p && distance <= 20 && d.saturating_sub(dist + distance) >= 100
                })
                .count()
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    #[test]
    fn step1_first_example() {
        assert_eq!(step1(INPUT), Answer::Unsigned(16));
    }
}
