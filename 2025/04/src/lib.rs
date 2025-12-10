use common::{Answer, BooleanGrid, Coordinates};

type IntType = i16;
type RollGrid = BooleanGrid<IntType>;
type Coords = Coordinates<IntType>;

fn parse(s: &str) -> RollGrid {
    let mut grid = RollGrid::new();

    s.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            if c == '@' {
                grid.mark(Coordinates::new(x as IntType, y as IntType))
            }
        })
    });

    grid
}

fn removeable_rolls(grid: &RollGrid) -> Vec<Coords> {
    grid.iter()
        .filter(|pos| {
            pos.neighbors_8()
                .iter()
                .filter(|pos| grid.contains(pos))
                .count()
                < 4
        })
        .copied()
        .collect()
}

pub fn step1(s: &str) -> Answer {
    let grid = parse(s);

    removeable_rolls(&grid).len().into()
}

pub fn step2(s: &str) -> Answer {
    let mut grid = parse(s);
    let mut removeable = 0;

    loop {
        let rolls = removeable_rolls(&grid);

        if rolls.is_empty() {
            break;
        }

        removeable += rolls.len();
        rolls.iter().for_each(|pos| grid.remove(pos));
    }

    removeable.into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn step1_finds_corret_sample_answer() {
        assert_eq!(step1(INPUT), Answer::Unsigned(13));
    }
}
