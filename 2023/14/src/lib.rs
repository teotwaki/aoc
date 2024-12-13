use common::Answer;
use rustc_hash::FxHashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Place {
    Rock,
    Block,
    Nothing,
}

impl From<char> for Place {
    fn from(c: char) -> Self {
        use Place::*;
        match c {
            'O' => Rock,
            '#' => Block,
            _ => Nothing,
        }
    }
}

#[derive(Debug)]
struct Map {
    rows: Vec<Vec<Place>>,
    cache: FxHashMap<Vec<Vec<Place>>, usize>,
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        Map {
            rows: s
                .lines()
                .map(|l| l.chars().map(Place::from).collect())
                .collect(),
            cache: FxHashMap::default(),
        }
    }
}

impl Map {
    fn tilt(&mut self) {
        for i in 1..self.rows.len() {
            for j in 0..self.rows[i].len() {
                if self.rows[i][j] == Place::Rock {
                    for dst in (0..i).rev() {
                        if self.rows[dst][j] != Place::Nothing {
                            if dst != i - 1 {
                                self.rows[dst + 1][j] = Place::Rock;
                                self.rows[i][j] = Place::Nothing;
                            }
                            break;
                        } else if dst == 0 {
                            self.rows[0][j] = Place::Rock;
                            self.rows[i][j] = Place::Nothing;
                        }
                    }
                }
            }
        }
    }

    fn rotate(&mut self) {
        let length = self.rows.len();

        let mut i = 0;

        while i < length {
            let mut j = i;

            while j < length {
                let tmp = self.rows[i][j];
                self.rows[i][j] = self.rows[j][i];
                self.rows[j][i] = tmp;

                j += 1;
            }

            i += 1;
        }

        self.rows.iter_mut().for_each(|row| row.reverse());
    }

    fn spin_cycle(&mut self, count: usize) {
        let mut i = 0;
        while i < count {
            self.tilt(); // tilt north

            self.rotate();
            self.tilt(); // tilt west

            self.rotate();
            self.tilt(); // tilt south

            self.rotate();
            self.tilt(); // tilt east

            self.rotate(); // original orientation (north)

            if let Some(n) = self.cache.get(&self.rows) {
                let jump = (count - i) % (i - n);
                i = count - jump;
            }

            self.cache.insert(self.rows.to_vec(), i);

            i += 1;
        }
    }

    fn weight(&self) -> usize {
        self.rows
            .iter()
            .rev()
            .enumerate()
            .rev()
            .map(|(i, row)| row.iter().filter(|p| **p == Place::Rock).count() * (i + 1))
            .sum()
    }
}

pub fn step1(s: &str) -> Answer {
    let mut map = Map::from(s);

    map.tilt();
    map.weight().into()
}

pub fn step2(s: &str) -> Answer {
    let mut map = Map::from(s);

    map.spin_cycle(1_000_000_000);

    map.weight().into()
}
