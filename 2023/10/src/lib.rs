use common::Answer;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Entry {
    Start,
    Empty,
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Entry {
    fn connects_bottom(&self, other: Self) -> bool {
        match self {
            Self::Start | Self::Vertical | Self::SouthEast | Self::SouthWest => matches!(
                other,
                Self::Start | Self::Vertical | Self::NorthEast | Self::NorthWest
            ),
            _ => false,
        }
    }

    fn connects_top(&self, other: Self) -> bool {
        match self {
            Self::Start | Self::Vertical | Self::NorthEast | Self::NorthWest => matches!(
                other,
                Self::Start | Self::Vertical | Self::SouthEast | Self::SouthWest
            ),
            _ => false,
        }
    }

    fn connects_left(&self, other: Self) -> bool {
        match self {
            Self::Start | Self::Horizontal | Self::NorthWest | Self::SouthWest => matches!(
                other,
                Self::Start | Self::Horizontal | Self::NorthEast | Self::SouthEast
            ),
            _ => false,
        }
    }

    fn connects_right(&self, other: Self) -> bool {
        match self {
            Self::Start | Self::Horizontal | Self::NorthEast | Self::SouthEast => matches!(
                other,
                Self::Start | Self::Horizontal | Self::NorthWest | Self::SouthWest
            ),
            _ => false,
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Entry::Start => '▣',
                Entry::Empty => ' ',
                Entry::Horizontal => '─',
                Entry::Vertical => '│',
                Entry::NorthEast => '└',
                Entry::NorthWest => '┘',
                Entry::SouthEast => '┌',
                Entry::SouthWest => '┐',
            }
        )
    }
}

impl From<char> for Entry {
    fn from(c: char) -> Self {
        match c {
            'S' => Entry::Start,
            '.' => Entry::Empty,
            '-' => Entry::Horizontal,
            '|' => Entry::Vertical,
            'L' => Entry::NorthEast,
            'J' => Entry::NorthWest,
            'F' => Entry::SouthEast,
            '7' => Entry::SouthWest,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<Entry>>,
}

impl Map {
    fn connections(&self, coords: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = coords;
        let rows = self.map.len();
        let columns = self.map[0].len();
        let entry = self.map[x][y];

        let mut coords = vec![];

        if x > 0 && entry.connects_top(self.map[x - 1][y]) {
            coords.push((x - 1, y));
        }

        if x < (rows - 1) && entry.connects_bottom(self.map[x + 1][y]) {
            coords.push((x + 1, y));
        }

        if y > 0 && entry.connects_left(self.map[x][y - 1]) {
            coords.push((x, y - 1));
        }

        if y < (columns - 1) && entry.connects_right(self.map[x][y + 1]) {
            coords.push((x, y + 1));
        }

        coords
    }

    fn start_node(&self) -> (usize, usize) {
        *self
            .map
            .iter()
            .enumerate()
            .filter_map(|(x, line)| {
                let line: Vec<_> = line
                    .iter()
                    .enumerate()
                    .filter_map(|(y, e)| if *e == Entry::Start { Some(y) } else { None })
                    .collect();

                if line.len() == 1 {
                    Some((x, *line.first().unwrap()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .first()
            .unwrap()
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let map = s
            .lines()
            .map(|l| l.chars().map(Entry::from).collect::<Vec<_>>())
            .collect();

        Map { map }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in &self.map {
            for entry in line {
                write!(f, "{}", entry)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn get_path(s: &str) -> Vec<(usize, usize)> {
    let map = Map::from(s);

    let start = map.start_node();
    let mut path = vec![start];

    loop {
        let conns: Vec<_> = map
            .connections(*path.last().unwrap())
            .into_iter()
            .filter(|coords| !path.contains(coords))
            .collect();

        if conns.is_empty() {
            path.push(start);
            break;
        }

        path.push(conns[0]);
    }

    path
}

pub fn step1(s: &str) -> Answer {
    let path = get_path(s);

    (path.len() / 2).into()
}

pub fn step2(s: &str) -> Answer {
    let path = get_path(s);

    let area = path
        .windows(2)
        .map(|w| {
            let (x1, y1) = w[0];
            let (x2, y2) = w[1];

            (x1 * y2) as i32 - (x2 * y1) as i32
        })
        .sum::<i32>()
        .abs()
        / 2;

    (area + 1 - (path.len() / 2) as i32).into()
}
