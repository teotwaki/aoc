use common::{Answer, Coordinates, Grid};
use std::collections::HashMap;

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

fn do_bfs(map: &Map, f: BFSFunc) -> usize {
    let mut scores: HashMap<Coordinates<i8>, usize> = HashMap::new();

    map.iter()
        .filter(|(_, v)| **v == 0)
        .map(|(pos, _)| *pos)
        .flat_map(|start| {
            f(
                map,
                start,
                |_, value| *value == 9,
                |(_, &a), (_, &b)| a < b && b - a <= 1,
            )
        })
        .for_each(|trail| {
            let head = *trail.first().unwrap();
            *scores.entry(head).or_default() += 1;
        });

    scores.values().sum::<usize>()
}

pub fn step1(s: &str) -> Answer {
    let map = parse(s);
    do_bfs(&map, Map::bfs_all).into()
}

pub fn step2(s: &str) -> Answer {
    let map = parse(s);
    do_bfs(&map, Map::bfs_exhaustive).into()
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
