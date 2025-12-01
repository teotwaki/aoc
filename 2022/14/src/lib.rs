use common::Answer;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map,
    multi::separated_list1,
};
use rustc_hash::FxHashMap;

#[derive(Debug, PartialEq)]
enum Item {
    Sand,
    Rock,
    SandProducer,
}

#[derive(Debug)]
struct Map {
    columns: FxHashMap<i32, FxHashMap<i32, Item>>,
}

impl Map {
    fn new() -> Self {
        Map {
            columns: FxHashMap::default(),
        }
    }

    fn get(&self, loc: Point) -> Option<&Item> {
        self.columns.get(&loc.x).and_then(|c| c.get(&loc.y))
    }

    fn insert(&mut self, loc: Point, item: Item) {
        self.columns.entry(loc.x).or_default().insert(loc.y, item);
    }

    fn insert_wall(&mut self, wall: Wall) {
        match ((wall.from.x, wall.from.y), (wall.to.x, wall.to.y)) {
            ((x1, y1), (x2, y2)) if x1 == x2 && y1 < y2 => {
                for i in y1..=y2 {
                    self.insert(Point { x: x1, y: i }, Item::Rock);
                }
            }
            ((x1, y1), (x2, y2)) if x1 == x2 && y1 > y2 => {
                for i in y2..=y1 {
                    self.insert(Point { x: x1, y: i }, Item::Rock);
                }
            }
            ((x1, y1), (x2, y2)) if y1 == y2 && x1 < x2 => {
                for i in x1..=x2 {
                    self.insert(Point { x: i, y: y1 }, Item::Rock);
                }
            }
            ((x1, y1), (x2, y2)) if y1 == y2 && x1 > x2 => {
                for i in x2..=x1 {
                    self.insert(Point { x: i, y: y1 }, Item::Rock);
                }
            }
            _ => unreachable!(),
        }
    }

    fn produce(&mut self) -> bool {
        let mut location = Point { x: 500, y: 0 };

        loop {
            let cell = self.get(location);

            if cell.is_none() || cell == Some(&Item::SandProducer) {
                location.down();

                if location.is_out_of_bounds() {
                    break false;
                }
            } else {
                location.left();

                if self.get(location).is_some() {
                    location.double_right();
                    if self.get(location).is_some() {
                        location.revert();

                        if self.get(location) == Some(&Item::SandProducer) {
                            self.insert(location, Item::Sand);
                            break false;
                        } else {
                            self.insert(location, Item::Sand);
                            break true;
                        }
                    }
                }
            }
        }
    }

    fn count_sand(&self) -> usize {
        self.columns
            .values()
            .flat_map(|c| c.values())
            .filter(|v| **v == Item::Sand)
            .count()
    }

    fn min_max_x(&self) -> (i32, i32) {
        let keys = self.columns.keys();
        let min = keys.clone().min().expect("Couldn't find minimum x value");
        let max = keys.max().expect("Couldn't find maximum x value");

        (*min, *max)
    }

    fn min_max_y(&self) -> (i32, i32) {
        let keys = self.columns.iter().flat_map(|(_, c)| c.keys());
        let min = keys.clone().min().expect("Couldn't find minimum y value");
        let max = keys.max().expect("Couldn't find maximum y value");

        (*min, *max)
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[inline]
    fn down(&mut self) {
        self.y += 1;
    }

    #[inline]
    fn left(&mut self) {
        self.x -= 1;
    }

    #[inline]
    fn double_right(&mut self) {
        self.x += 2;
    }

    #[inline]
    fn revert(&mut self) {
        self.x -= 1;
        self.y -= 1;
    }

    #[inline]
    fn is_out_of_bounds(&self) -> bool {
        self.y > 200
    }
}

struct Wall {
    from: Point,
    to: Point,
}

fn parse_i32(s: &str) -> IResult<&str, i32> {
    map(digit1, |i: &str| i.parse().expect("Invalid number")).parse(s)
}

fn parse_point(s: &str) -> IResult<&str, Point> {
    let (s, (x, _, y)) = (parse_i32, tag(","), parse_i32).parse(s)?;

    Ok((s, Point { x, y }))
}

fn parse_line(s: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), parse_point).parse(s)
}

fn parse(s: &str) -> Vec<Vec<Point>> {
    let (_, lines) = separated_list1(newline, parse_line)
        .parse(s)
        .expect("Failed to parse lines");

    lines
}

fn parse_map(s: &str) -> Map {
    let mut map = Map::new();

    map.insert(Point { x: 500, y: 0 }, Item::SandProducer);

    parse(s).iter().for_each(|points| {
        points.windows(2).for_each(|points| {
            map.insert_wall(Wall {
                from: points[0],
                to: points[1],
            })
        });
    });

    map
}

pub fn step1(s: &str) -> Answer {
    let mut map = parse_map(s);

    while map.produce() {}

    map.count_sand().into()
}

pub fn step2(s: &str) -> Answer {
    let mut map = parse_map(s);

    let (min_x, max_x) = map.min_max_x();
    let (_, max_y) = map.min_max_y();

    map.insert_wall(Wall {
        from: Point {
            x: min_x - 180,
            y: max_y + 2,
        },
        to: Point {
            x: max_x + 180,
            y: max_y + 2,
        },
    });

    while map.produce() {}

    map.count_sand().into()
}
