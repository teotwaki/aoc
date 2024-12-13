use common::{Answer, Coordinates, Grid};
use rayon::prelude::*;
use rustc_hash::FxHashMap;

type Map = Grid<i8, u8>;
type Height = u8;
type Coords = Coordinates<i8>;
type Path = Vec<Coords>;
type CellRef<'a> = (Coords, &'a Height);
type EndValidator = fn(Coords, &Height) -> bool;
type NeighborValidator = fn(CellRef, CellRef) -> bool;
type BFSFunc = fn(&Map, Coords, EndValidator, NeighborValidator) -> Vec<Path>;

fn parse(s: &str) -> Map {
    let mut map = Map::new();

    s.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            let val = c.to_digit(10).unwrap();
            map.store((x, y).into(), val as u8);
        })
    });

    map
}

#[inline]
fn is_trailhead((_, height): &(&Coords, &u8)) -> bool {
    **height == 0
}

#[inline]
fn is_trailend(_: Coords, value: &u8) -> bool {
    *value == 9
}

#[inline]
fn is_trail((_, &a): CellRef, (_, &b): CellRef) -> bool {
    a < b && b - a <= 1
}

fn count_trails(map: &Map, f: BFSFunc) -> usize {
    map.iter()
        .par_bridge()
        .filter(is_trailhead)
        .map(|(pos, _)| *pos)
        .flat_map(|start| f(map, start, is_trailend, is_trail))
        .fold(FxHashMap::<Coords, usize>::default, |mut acc, trail| {
            let head = *trail.first().unwrap();
            *acc.entry(head).or_default() += 1;

            acc
        })
        .map(|acc| acc.values().sum::<usize>())
        .sum()
}

pub fn step1(s: &str) -> Answer {
    let map = parse(s);
    count_trails(&map, Map::bfs_all).into()
}

pub fn step2(s: &str) -> Answer {
    let map = parse(s);
    count_trails(&map, Map::bfs_exhaustive).into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn step1_finds_correct_answer() {
        assert_eq!(step1(INPUT), Answer::Unsigned(36));
    }

    #[test]
    fn step2_finds_correct_answer() {
        assert_eq!(step2(INPUT), Answer::Unsigned(81));
    }
}
