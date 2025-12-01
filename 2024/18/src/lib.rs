use common::{Answer, BooleanGrid, Coordinates};

type IntType = i8;
type Memory = BooleanGrid<IntType>;
type Coords = Coordinates<IntType>;

fn parse(s: &str) -> impl Iterator<Item = Coords> + '_ {
    s.lines().map(|s| Coordinates::try_from(s).unwrap())
}

fn find_shortest_path(s: &str, obstacles: usize, max: Coords) -> usize {
    let mut mem = Memory::new();
    parse(s).take(obstacles).for_each(|pos| mem.mark(pos));
    mem.bfs((0, 0).into(), max, false).len() - 1
}

pub fn step1(s: &str) -> Answer {
    find_shortest_path(s, 1024, (70, 70).into()).into()
}

fn find_first_blocker(s: &str, obstacles: usize, max: Coords) -> Coords {
    let mut mem = Memory::new();
    let mut coords = parse(s);
    (0..obstacles).for_each(|_| mem.mark(coords.next().unwrap()));

    loop {
        let pos = coords.next().unwrap();
        mem.mark(pos);

        if mem.bfs((0, 0).into(), max, false).is_empty() {
            return pos;
        }
    }
}

pub fn step2(s: &str) -> Answer {
    let coords = find_first_blocker(s, 1024, (70, 70).into());

    format!("{},{}", coords.x(), coords.y()).into()
}

#[cfg(test)]
mod test_2024_18 {
    use super::*;

    const INPUT: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    #[test]
    fn find_shortest_path_finds_example_solution() {
        assert_eq!(find_shortest_path(INPUT, 12, (6, 6).into()), 22);
    }
}
