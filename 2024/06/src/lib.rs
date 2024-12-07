use common::{Answer, Coordinates, Direction, Grid};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

type IntType = i16;
type Coords = Coordinates<IntType>;

fn parse(s: &str) -> (Grid<(), IntType>, Coords) {
    let mut map = Grid::new();
    let mut guard = None;

    s.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            let pos = (x as IntType, y as IntType).into();
            match c {
                '^' => guard = Some(pos),
                '#' => map.store(pos, ()),
                _ => {}
            }
        })
    });

    (map, guard.unwrap())
}

enum Outcome {
    Loop,
    OutOfBounds(HashSet<(Coords, Direction)>),
}

fn simulate_route(obstacles: &Grid<(), IntType>, start: Coords) -> Outcome {
    let mut guard_locations = HashSet::new();
    let mut direction = Direction::Up;
    let mut guard_pos = start;

    guard_locations.insert((start, direction));

    loop {
        let next = guard_pos.next(direction);

        if obstacles.get(&next).is_some() {
            direction.turn_clockwise();
            guard_locations.insert((guard_pos, direction));
            continue;
        }

        if guard_locations.contains(&(next, direction)) {
            break Outcome::Loop;
        }

        if !obstacles.within_bounds(next) {
            break Outcome::OutOfBounds(guard_locations);
        }

        guard_pos = next;
        guard_locations.insert((guard_pos, direction));
    }
}

pub fn step1(s: &str) -> Answer {
    let (obstacles, start) = parse(s);

    let path = match simulate_route(&obstacles, start) {
        Outcome::OutOfBounds(path) => path,
        _ => unreachable!(),
    };

    path.iter().map(|&(pos, _)| pos).unique().count().into()
}

pub fn step2(s: &str) -> Answer {
    let (obstacles, start) = parse(s);

    let guard_path = match simulate_route(&obstacles, start) {
        Outcome::OutOfBounds(locations) => locations,
        _ => unreachable!(),
    };

    guard_path
        .par_iter()
        .filter(|&&(coords, mut dir)| {
            let mut pos = coords.previous(dir);
            dir.turn_clockwise();

            loop {
                let next = pos.next(dir);

                if !obstacles.within_bounds(next) {
                    break false;
                }

                if obstacles.get(&next).is_some() {
                    break true;
                }

                pos = next;
            }
        })
        .map(|&(coords, _)| coords)
        .collect::<HashSet<_>>()
        .par_iter()
        .filter(|&pos| *pos != start)
        .filter(|&&pos| {
            let mut obstacles = obstacles.clone();
            obstacles.store(pos, ());

            matches!(simulate_route(&obstacles, start), Outcome::Loop)
        })
        .count()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn step1_finds_correct_answer() {
        assert_eq!(step1(INPUT), Answer::Unsigned(41));
    }

    #[test]
    fn step2_finds_correct_answer() {
        assert_eq!(step2(INPUT), Answer::Unsigned(6));
    }
}
