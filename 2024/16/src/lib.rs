use common::{Answer, BooleanGrid, Coordinates, Direction};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{cmp::Ordering, collections::BinaryHeap};

type IntType = i16;
type Map = BooleanGrid<IntType>;
type Coords = Coordinates<IntType>;

fn parse(s: &str) -> (Map, Coords, Coords) {
    let mut map = Map::new();
    let mut start = (0, 0).into();
    let mut end = (0, 0).into();

    s.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '.' | 'S' | 'E' => {
                map.mark((x, y).into());
                if c == 'S' {
                    start = (x, y).into();
                } else if c == 'E' {
                    end = (x, y).into();
                }
            }
            _ => {}
        });
    });

    (map, start, end)
}

fn calculate_direction(a: Coords, b: Coords) -> Option<Direction> {
    use Direction::*;
    use Ordering::*;

    match a.x().cmp(&b.x()) {
        Greater => Some(Left),
        Less => Some(Right),
        _ => match a.y().cmp(&b.y()) {
            Greater => Some(Up),
            Less => Some(Down),
            _ => None,
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    cost: usize,
    position: Coords,
    direction: Direction,
    path: Option<Vec<Coords>>,
}

impl State {
    fn next(&self, pos: Coords) -> Self {
        let dir = calculate_direction(self.position, pos).unwrap();

        State {
            cost: self.cost + if dir == self.direction { 1 } else { 1001 },
            direction: dir,
            position: pos,
            path: None,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(map: &Map, start: Coords, end: Coords) -> Option<usize> {
    let mut distances = FxHashMap::from_iter(map.iter().map(|&pos| (pos, usize::MAX)));
    let mut heap = BinaryHeap::new();

    distances.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
        direction: Direction::Right,
        path: None,
    });

    while let Some(state) = heap.pop() {
        if state.position == end {
            return Some(state.cost);
        }

        if state.cost > *distances.get(&state.position).unwrap() {
            continue;
        }

        for neighbor in state.position.neighbors() {
            if map.within_bounds(neighbor) && map.contains(&neighbor) {
                let next = state.next(neighbor);

                if next.cost < *distances.get(&next.position).unwrap() {
                    distances.insert(next.position, next.cost);
                    heap.push(next);
                }
            }
        }
    }

    None
}

pub fn step1(s: &str) -> Answer {
    let (map, start, end) = parse(s);
    dijkstra(&map, start, end).unwrap().into()
}

fn dijkstra_all(map: &Map, start: Coords, end: Coords) -> (usize, Vec<Vec<Coords>>) {
    use Direction::*;

    let directions = [Up, Right, Down, Left];
    let mut distances = FxHashMap::from_iter(
        map.iter()
            .flat_map(|pos| directions.iter().map(|dir| ((*pos, *dir), usize::MAX))),
    );

    distances.insert((start, Right), 0);
    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: start,
        direction: Right,
        path: Some(vec![start]),
    });

    let mut best_cost = usize::MAX;
    let mut best_paths = Vec::new();

    while let Some(state) = heap.pop() {
        if state.position == end {
            if state.cost < best_cost {
                best_cost = state.cost;
                best_paths.clear();
            }

            if state.cost == best_cost {
                best_paths.push(state.path.clone().unwrap());
            }

            continue;
        }

        if state.cost > *distances.get(&(state.position, state.direction)).unwrap() {
            continue;
        }

        [
            state.direction.counterclockwise(),
            state.direction.clockwise(),
        ]
        .iter()
        .for_each(|&dir| {
            let cost = state.cost + 1000;
            if cost <= distances[&(state.position, dir)] {
                if cost < distances[&(state.position, dir)] {
                    distances.insert((state.position, dir), cost);
                }

                heap.push(State {
                    cost,
                    position: state.position,
                    direction: dir,
                    path: state.path.clone(),
                });
            }
        });

        let pos = state.position.next(state.direction);
        if map.within_bounds(pos) && map.contains(&pos) {
            let cost = state.cost + 1;
            if cost <= distances[&(pos, state.direction)] {
                if cost < distances[&(pos, state.direction)] {
                    distances.insert((pos, state.direction), cost);
                }

                let mut path = state.path.clone().unwrap();
                path.push(pos);

                heap.push(State {
                    cost,
                    position: pos,
                    direction: state.direction,
                    path: Some(path),
                });
            }
        }
    }

    (best_cost, best_paths)
}

pub fn step2(s: &str) -> Answer {
    let (map, start, end) = parse(s);
    let (_, paths) = dijkstra_all(&map, start, end);

    paths
        .iter()
        .flat_map(|path| path.iter())
        .collect::<FxHashSet<_>>()
        .len()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    const FIRST_EXAMPLE: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    const SECOND_EXAMPLE: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    #[test]
    fn step1_finds_correct_answer_first_example() {
        assert_eq!(step1(FIRST_EXAMPLE), Answer::Unsigned(7036));
    }

    #[test]
    fn step1_finds_correct_answer_second_example() {
        assert_eq!(step1(SECOND_EXAMPLE), Answer::Unsigned(11048));
    }

    #[test]
    fn step2_finds_correct_answer_first_example() {
        assert_eq!(step2(FIRST_EXAMPLE), Answer::Unsigned(45));
    }

    #[test]
    fn step2_finds_correct_answer_second_example() {
        assert_eq!(step2(SECOND_EXAMPLE), Answer::Unsigned(64));
    }
}
