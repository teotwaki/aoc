use common::Answer;
use itertools::Itertools;
use std::cmp::max;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn adjacents(&self) -> Vec<Coords> {
        let mut adjacents = vec![
            Coords {
                x: self.x,
                y: self.y + 1,
            },
            Coords {
                x: self.x + 1,
                y: self.y + 1,
            },
            Coords {
                x: self.x + 1,
                y: self.y,
            },
        ];

        if self.x > 0 {
            adjacents.push(Coords {
                x: self.x - 1,
                y: self.y,
            });

            adjacents.push(Coords {
                x: self.x - 1,
                y: self.y + 1,
            });
        }

        if self.y > 0 {
            adjacents.push(Coords {
                x: self.x,
                y: self.y - 1,
            });

            adjacents.push(Coords {
                x: self.x + 1,
                y: self.y - 1,
            });

            if self.x > 0 {
                adjacents.push(Coords {
                    x: self.x - 1,
                    y: self.y - 1,
                });
            }
        }

        adjacents
    }
}

#[derive(Debug, Clone, Copy)]
struct Number {
    value: u32,
    coords: Coords,
    length: usize,
}

impl Number {
    fn all_coords(&self) -> Vec<Coords> {
        (self.coords.y..(self.coords.y + self.length))
            .map(|y| Coords {
                x: self.coords.x,
                y,
            })
            .collect()
    }

    fn adjacents(&self) -> Vec<Coords> {
        self.all_coords()
            .iter()
            .flat_map(|c| c.adjacents())
            .unique()
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
struct Symbol {
    coords: Coords,
    is_gear: bool,
}

fn flush(numbers: &mut Vec<Number>, number: &mut String, coords: Coords) {
    if !number.is_empty() {
        let n = number.parse().unwrap();

        numbers.push(Number {
            value: n,
            coords: Coords {
                x: coords.x,
                y: coords.y - number.len(),
            },
            length: number.len(),
        });

        number.clear();
    }
}

fn parse(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers = vec![];
    let mut symbols = vec![];
    let mut number = String::new();
    let mut line_length = 0;

    input.lines().enumerate().for_each(|(x, l)| {
        line_length = max(line_length, l.len());

        l.chars().enumerate().for_each(|(y, c)| {
            let coords = Coords { x, y };
            match c {
                '.' => flush(&mut numbers, &mut number, coords),
                '0'..='9' => number.push(c),
                c => {
                    flush(&mut numbers, &mut number, coords);
                    symbols.push(Symbol {
                        coords,
                        is_gear: c == '*',
                    });
                }
            }
        });

        flush(
            &mut numbers,
            &mut number,
            Coords {
                x,
                y: line_length - 1,
            },
        );
    });

    (numbers, symbols)
}

pub fn step1(s: &str) -> Answer {
    let (numbers, symbols) = parse(s);

    numbers
        .iter()
        .filter(|n| {
            n.adjacents()
                .iter()
                .any(|coords| symbols.iter().any(|s| s.coords == *coords))
        })
        .map(|n| n.value)
        .sum::<u32>()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let (numbers, symbols) = parse(s);

    symbols
        .iter()
        .filter(|s| s.is_gear)
        .filter_map(|s| {
            let v: Vec<_> = numbers
                .iter()
                .filter(|n| n.adjacents().iter().any(|c| c == &s.coords))
                .map(|n| n.value)
                .collect();

            if v.len() == 2 {
                Some(v.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum::<u32>()
        .into()
}
