use common::{Answer, Coordinates, Grid};

fn parse(s: &str) -> Grid<u8, char> {
    let mut grid = Grid::new();
    s.lines().enumerate().for_each(|(y, l)| {
        l.chars()
            .enumerate()
            .for_each(|(x, c)| grid.store((x as u8, y as u8).into(), c))
    });

    grid
}

fn possibilities(pos: Coordinates<u8>) -> Vec<[Coordinates<u8>; 4]> {
    let mut possibilities = vec![
        [
            pos,
            pos.right(),
            pos.right().right(),
            pos.right().right().right(),
        ],
        [pos, pos.down(), pos.down().down(), pos.down().down().down()],
        [
            pos,
            pos.southeast(),
            pos.southeast().southeast(),
            pos.southeast().southeast().southeast(),
        ],
    ];

    if pos.x() >= 3 {
        possibilities.push([pos, pos.left(), pos.left().left(), pos.left().left().left()]);
        possibilities.push([
            pos,
            pos.southwest(),
            pos.southwest().southwest(),
            pos.southwest().southwest().southwest(),
        ]);

        if pos.y() >= 3 {
            possibilities.push([
                pos,
                pos.northwest(),
                pos.northwest().northwest(),
                pos.northwest().northwest().northwest(),
            ]);
        }
    }

    if pos.y() >= 3 {
        possibilities.push([pos, pos.up(), pos.up().up(), pos.up().up().up()]);
        possibilities.push([
            pos,
            pos.northeast(),
            pos.northeast().northeast(),
            pos.northeast().northeast().northeast(),
        ]);
    }

    possibilities
}

fn validate_xmas(grid: &Grid<u8, char>, candidate: &[Coordinates<u8>]) -> bool {
    grid.get(&candidate[1]) == Some(&'M')
        && grid.get(&candidate[2]) == Some(&'A')
        && grid.get(&candidate[3]) == Some(&'S')
}

pub fn step1(s: &str) -> Answer {
    let grid = parse(s);

    grid.iter()
        .filter(|(_, c)| **c == 'X')
        .flat_map(|(&pos, _)| {
            possibilities(pos)
                .into_iter()
                .filter(|cand| validate_xmas(&grid, cand))
        })
        .count()
        .into()
}

fn validate_mas(grid: &Grid<u8, char>, pos: Coordinates<u8>) -> bool {
    let nw = *grid.get(&pos.northwest()).unwrap_or(&' ');
    let ne = *grid.get(&pos.northeast()).unwrap_or(&' ');
    let se = *grid.get(&pos.southeast()).unwrap_or(&' ');
    let sw = *grid.get(&pos.southwest()).unwrap_or(&' ');

    (nw == 'S' && se == 'M' || nw == 'M' && se == 'S')
        && (ne == 'S' && sw == 'M' || ne == 'M' && sw == 'S')
}

pub fn step2(s: &str) -> Answer {
    let grid = parse(s);

    grid.iter()
        .filter(|(pos, c)| **c == 'A' && pos.x() > 0 && pos.y() > 0 && validate_mas(&grid, **pos))
        .count()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn parse_extracts_correct_number_of_rows() {
        assert_eq!(parse(INPUT).height(), 10);
    }

    #[test]
    fn parse_extracts_correct_number_of_columns() {
        assert_eq!(parse(INPUT).width(), 10);
    }

    #[test]
    fn step1_finds_correct_answer() {
        assert_eq!(step1(INPUT), Answer::Unsigned(18));
    }

    #[test]
    fn step2_finds_correct_answer() {
        assert_eq!(step2(INPUT), Answer::Unsigned(9));
    }
}
