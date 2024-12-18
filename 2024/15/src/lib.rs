use common::{Answer, Coordinates, Direction, Grid};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Cell {
    #[default]
    Box,
    BoxLeft,
    BoxRight,
    Wall,
    Robot,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        use Cell::*;

        match value {
            'O' => Box,
            '[' => BoxLeft,
            ']' => BoxRight,
            '#' => Wall,
            '@' => Robot,
            _ => unreachable!(),
        }
    }
}

type IntType = i16;
type Coords = Coordinates<IntType>;
type WarehouseMap = Grid<IntType, Cell>;
type Moves = VecDeque<Direction>;

fn parse_warehouse(s: &str) -> Warehouse {
    let mut warehouse_map = WarehouseMap::new();

    s.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|&(_, c)| c != '.')
            .for_each(|(x, c)| warehouse_map.store((x, y).into(), c.into()))
    });

    Warehouse::new(warehouse_map)
}

fn parse_moves(s: &str) -> Moves {
    s.chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => unreachable!(),
        })
        .collect()
}

fn parse(s: &str) -> (Warehouse, Moves) {
    let mut parts = s.split("\n\n");

    (
        parse_warehouse(parts.next().unwrap()),
        parse_moves(parts.next().unwrap()),
    )
}

#[derive(Debug, Clone)]
struct Warehouse {
    map: WarehouseMap,
    robot: Coords,
    is_wide: bool,
}

impl Warehouse {
    fn new(mut map: WarehouseMap) -> Self {
        let robot = *map.find(&Cell::Robot).unwrap();
        let is_wide = map.find(&Cell::BoxLeft).is_some();
        map.remove(&robot);

        Self {
            map,
            robot,
            is_wide,
        }
    }

    fn find_end(&self, mut pos: Coords, dir: Direction) -> Option<Coords> {
        use Cell::*;

        loop {
            match self.map.get(&pos) {
                Some(Wall) => break None,
                None => break Some(pos),
                Some(Box) | Some(BoxLeft) | Some(BoxRight) => pos.move_next(dir),
                Some(Robot) => unreachable!(),
            };
        }
    }

    fn is_move_possible(&self, destination: Coords, dir: Direction) -> bool {
        use Cell::*;

        match self.map.get(&destination) {
            Some(BoxLeft) => {
                match (
                    self.map.get(&destination.next(dir)),
                    self.map.get(&destination.next(dir).right()),
                ) {
                    (None, None) => true,
                    (Some(BoxLeft), Some(BoxRight)) => {
                        self.is_move_possible(destination.next(dir), dir)
                    }
                    (Some(BoxRight), Some(BoxLeft)) => {
                        self.is_move_possible(destination.next(dir).left(), dir)
                            && self.is_move_possible(destination.next(dir).right(), dir)
                    }
                    (Some(BoxRight), None) => {
                        self.is_move_possible(destination.next(dir).left(), dir)
                    }
                    (None, Some(BoxLeft)) => {
                        self.is_move_possible(destination.next(dir).right(), dir)
                    }
                    _ => false,
                }
            }
            Some(BoxRight) => self.is_move_possible(destination.left(), dir),
            None => true,
            _ => false,
        }
    }

    fn move_box_vertical(&mut self, destination: Coords, dir: Direction) {
        use Cell::*;

        match self.map.get(&destination) {
            Some(BoxLeft) => {
                match (
                    self.map.get(&destination.next(dir)),
                    self.map.get(&destination.next(dir).right()),
                ) {
                    (None, None) => {}
                    (Some(BoxLeft), Some(BoxRight)) => {
                        self.move_box_vertical(destination.next(dir), dir);
                    }
                    (Some(BoxRight), Some(BoxLeft)) => {
                        self.move_box_vertical(destination.next(dir).left(), dir);
                        self.move_box_vertical(destination.next(dir).right(), dir);
                    }
                    (Some(BoxRight), None) => {
                        self.move_box_vertical(destination.next(dir).left(), dir);
                    }
                    (None, Some(BoxLeft)) => {
                        self.move_box_vertical(destination.next(dir).right(), dir);
                    }
                    _ => return,
                }
                self.map.store(destination.next(dir), BoxLeft);
                self.map.store(destination.next(dir).right(), BoxRight);
                self.map.remove(&destination);
                self.map.remove(&destination.right());
            }
            Some(BoxRight) => self.move_box_vertical(destination.left(), dir),
            _ => {}
        }
    }

    fn move_robot(&mut self, dir: Direction) {
        let destination = self.robot.next(dir);

        if !self.is_wide {
            if let Some(end) = self.find_end(destination, dir) {
                self.map.store(end, Cell::Box);
                self.map.remove(&destination);
                self.robot = destination;
            }
        } else if dir == Direction::Left {
            if let Some(mut end) = self.find_end(destination, dir) {
                while end < destination {
                    self.map.store(end, Cell::BoxLeft);
                    self.map.store(end.right(), Cell::BoxRight);
                    end = end + (2, 0).into();
                }
                self.map.remove(&destination);
                self.robot = destination;
            }
        } else if dir == Direction::Right {
            if let Some(mut end) = self.find_end(destination, dir) {
                while destination < end {
                    self.map.store(end, Cell::BoxRight);
                    self.map.store(end.left(), Cell::BoxLeft);
                    end = end + (-2, 0).into();
                }
                self.map.remove(&destination);
                self.robot = destination;
            }
        } else if self.is_move_possible(destination, dir) {
            self.move_box_vertical(destination, dir);
            self.robot = destination;
        }
    }

    fn sum_gps_coords(&self) -> usize {
        self.map
            .iter()
            .filter(|&(_, cell)| cell == &Cell::Box || cell == &Cell::BoxLeft)
            .map(|(pos, _)| pos.x() as usize + pos.y() as usize * 100)
            .sum()
    }
}

pub fn step1(s: &str) -> Answer {
    let (mut warehouse, moves) = parse(s);

    moves.into_iter().for_each(|dir| warehouse.move_robot(dir));

    warehouse.sum_gps_coords().into()
}

pub fn step2(s: &str) -> Answer {
    let s = s
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");
    step1(&s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small_input_checks() {
        let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        let (mut warehouse, moves) = parse(input);

        moves.into_iter().for_each(|d| warehouse.move_robot(d));

        assert_eq!(warehouse.robot, (4, 4).into());
        assert_eq!(warehouse.sum_gps_coords(), 2028);
    }

    #[test]
    fn larger_input_check() {
        let input = r#"##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########"#;

        let warehouse = parse_warehouse(input);

        assert_eq!(warehouse.sum_gps_coords(), 10092);
    }

    #[test]
    fn step2_small_input_checks() {
        let input = r#"##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############

<vv<<^^<<^^"#;

        let (mut warehouse, moves) = parse(input);

        moves.into_iter().for_each(|d| warehouse.move_robot(d));

        assert_eq!(warehouse.robot, (5, 2).into());
        assert_eq!(warehouse.sum_gps_coords(), 618);
    }

    #[test]
    fn wide_gps_sum_calculation() {
        let input = r#"####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################"#;

        let warehouse = parse_warehouse(input);
        assert_eq!(warehouse.sum_gps_coords(), 9021);
    }

    #[test]
    fn wide_moves_up() {
        let input = r#"######
##..##
##[]##
##.@##
######"#;

        let mut warehouse = parse_warehouse(input);
        assert_eq!(warehouse.sum_gps_coords(), 202);

        warehouse.move_robot(Direction::Up);
        assert_eq!(warehouse.sum_gps_coords(), 102);

        warehouse.move_robot(Direction::Up);
        assert_eq!(warehouse.sum_gps_coords(), 102);
    }

    #[test]
    fn wide_moves_up2() {
        let input = r#"######
##..##
##[]##
##@.##
######"#;

        let mut warehouse = parse_warehouse(input);
        assert_eq!(warehouse.sum_gps_coords(), 202);

        warehouse.move_robot(Direction::Up);
        assert_eq!(warehouse.sum_gps_coords(), 102);

        warehouse.move_robot(Direction::Up);
        assert_eq!(warehouse.sum_gps_coords(), 102);
    }

    #[test]
    fn wide_moves_down() {
        let input = r#"######
##.@##
##[]##
##..##
######"#;

        let mut warehouse = parse_warehouse(input);
        assert_eq!(warehouse.sum_gps_coords(), 202);

        warehouse.move_robot(Direction::Down);
        assert_eq!(warehouse.sum_gps_coords(), 302);

        warehouse.move_robot(Direction::Down);
        assert_eq!(warehouse.sum_gps_coords(), 302);
    }

    #[test]
    fn wide_moves_down2() {
        let input = r#"######
##@.##
##[]##
##..##
######"#;

        let mut warehouse = parse_warehouse(input);
        assert_eq!(warehouse.sum_gps_coords(), 202);

        warehouse.move_robot(Direction::Down);
        assert_eq!(warehouse.sum_gps_coords(), 302);

        warehouse.move_robot(Direction::Down);
        assert_eq!(warehouse.sum_gps_coords(), 302);
    }

    #[test]
    fn wide_moves_double_up() {
        let input = r#"######
##..##
##[]##
##[]##
##.@##
######"#;

        let mut warehouse = parse_warehouse(input);
        assert_eq!(warehouse.sum_gps_coords(), 504);

        warehouse.move_robot(Direction::Up);
        assert_eq!(warehouse.sum_gps_coords(), 304);

        warehouse.move_robot(Direction::Up);
        assert_eq!(warehouse.sum_gps_coords(), 304);
    }
}
