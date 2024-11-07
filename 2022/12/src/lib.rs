use common::Answer;
use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

fn shortest_path_length(target: usize, previous: &HashMap<usize, usize>) -> usize {
    let mut u = target;
    let mut path = vec![];

    while previous.contains_key(&u) {
        path.push(u);
        u = previous[&u];
    }

    path.len()
}

fn find_item_with_smallest_distance(
    queue: &VecDeque<usize>,
    distances: &HashMap<usize, usize>,
) -> usize {
    queue
        .iter()
        .enumerate()
        .min_by(
            |(_, left), (_, right)| match (distances.get(left), distances.get(right)) {
                (Some(left), Some(right)) => left.cmp(right),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                _ => Ordering::Equal,
            },
        )
        .map(|(i, _)| i)
        .unwrap_or(0)
}

struct Map {
    map: Vec<char>,
    width: usize,
    height: usize,
    start: usize,
    end: usize,
}

impl Map {
    fn neighbours_of(&self, pos: usize) -> Vec<usize> {
        if pos >= self.map.len() {
            return vec![];
        }

        let mut candidates = Vec::with_capacity(4);

        if pos >= self.width {
            candidates.push(pos - self.width);
        }

        if pos < self.width * (self.height - 1) {
            candidates.push(pos + self.width);
        }

        if pos % self.width != 0 {
            candidates.push(pos - 1);
        }

        if pos % self.width != self.width - 1 {
            candidates.push(pos + 1);
        }

        candidates
            .into_iter()
            .filter(|p| {
                let left = self.map[pos] as i32;
                let right = self.map[*p] as i32;
                left.abs_diff(right) <= 1 || left < right
            })
            .collect()
    }

    fn new(s: &str) -> Self {
        let map: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

        let width = map[0].len();
        let height = map.len();

        let mut map: Vec<char> = map.into_iter().flatten().collect();
        let start = map.iter().position(|n| *n == 'S').unwrap();
        let end = map.iter().position(|n| *n == 'E').unwrap();

        map[start] = 'a';
        map[end] = 'z';

        Self {
            map,
            width,
            height,
            start,
            end,
        }
    }
}

fn solve_maze(s: &str) -> (Map, HashMap<usize, usize>) {
    let map = Map::new(s);

    let mut queue = VecDeque::from_iter(map.map.iter().enumerate().map(|(i, _)| i));
    let mut distances = HashMap::from([(map.end, 0)]);
    let mut previous = HashMap::new();

    while !queue.is_empty() {
        let pos = find_item_with_smallest_distance(&queue, &distances);
        let u = queue.remove(pos).expect("Couldn't unqueue item");

        if distances.contains_key(&u) {
            let distance = distances[&u] + 1;

            map.neighbours_of(u)
                .iter()
                .filter(|v| queue.contains(v))
                .for_each(|v| {
                    if distance < *distances.get(v).unwrap_or(&usize::MAX) {
                        distances.insert(*v, distance);
                        previous.insert(*v, u);
                    }
                });
        }
    }

    (map, previous)
}

pub fn step1(s: &str) -> Answer {
    let (map, previous) = solve_maze(s);

    let shortest_from_start = shortest_path_length(map.start, &previous);

    shortest_from_start.into()
}

pub fn step2(s: &str) -> Answer {
    let (map, previous) = solve_maze(s);

    let shortest_from_lowest = map
        .map
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == 'a')
        .map(|(i, _)| shortest_path_length(i, &previous))
        .filter(|n| *n != 0)
        .min()
        .unwrap();

    shortest_from_lowest.into()
}
