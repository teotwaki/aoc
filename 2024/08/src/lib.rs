use common::{Answer, BoundedGrid, Coordinates, Grid};
use std::collections::HashMap;

type IntType = i8;
type Coords = Coordinates<IntType>;
type FreqMap = HashMap<char, Grid<IntType, ()>>;

fn get_grid_size(s: &str) -> (Coords, Coords) {
    let max_x = s.lines().next().map(|l| l.chars().count()).unwrap();
    let max_y = s.lines().count();

    ((0, 0).into(), (max_x - 1, max_y - 1).into())
}

fn parse(s: &str) -> FreqMap {
    let mut frequencies = FreqMap::new();

    s.lines().enumerate().for_each(|(y, l)| {
        l.chars()
            .enumerate()
            .filter(|&(_, c)| c != '.')
            .for_each(|(x, c)| {
                frequencies
                    .entry(c)
                    .or_default()
                    .store(Coords::new(x as IntType, y as IntType), ())
            });
    });

    frequencies
}

fn store_antinodes(
    antinodes: &mut BoundedGrid<IntType, ()>,
    mut a: Coords,
    b: Coords,
    resonance: bool,
) {
    let diff = a - b;
    loop {
        a = a + diff;
        let inserted = antinodes.store(a, ());

        if !resonance || !inserted {
            break;
        }
    }
}

fn calculate_antinodes(s: &str, resonance: bool) -> usize {
    let (min, max) = get_grid_size(s);
    let mut antinodes = BoundedGrid::new(min, max);
    let frequencies = parse(s);

    frequencies.iter().for_each(|(_, antennas)| {
        antennas.iter().for_each(|(&pos, _)| {
            if resonance {
                antinodes.store(pos, ());
            }

            antennas
                .iter()
                .filter(|&(p, _)| *p != pos)
                .for_each(|(&other, _)| {
                    store_antinodes(&mut antinodes, pos, other, resonance);
                })
        })
    });

    antinodes.len()
}

pub fn step1(s: &str) -> Answer {
    calculate_antinodes(s, false).into()
}

pub fn step2(s: &str) -> Answer {
    calculate_antinodes(s, true).into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn step1_finds_correct_answer() {
        assert_eq!(step1(INPUT), Answer::Unsigned(14));
    }

    #[test]
    fn step2_finds_correct_answer() {
        assert_eq!(step2(INPUT), Answer::Unsigned(34));
    }
}
