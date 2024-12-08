use common::{Answer, BooleanGrid, Coordinates};

fn move_house(c: char, pos: &mut Coordinates<i32>) {
    match c {
        '^' => pos.move_up(),
        '>' => pos.move_right(),
        'v' => pos.move_down(),
        '<' => pos.move_left(),
        _ => unreachable!(),
    };
}

pub fn step1(s: &str) -> Answer {
    let mut grid = BooleanGrid::new();
    let mut pos = Coordinates::new(0, 0);

    grid.mark(pos);

    s.chars().for_each(|c| {
        move_house(c, &mut pos);
        grid.mark(pos);
    });

    grid.len().into()
}

pub fn step2(s: &str) -> Answer {
    let mut grid = BooleanGrid::new();
    let mut santa = Coordinates::new(0, 0);
    let mut robo = Coordinates::new(0, 0);

    grid.mark(santa);

    let mut chars = s.chars();

    loop {
        let Some(c) = chars.next() else {
            break;
        };

        move_house(c, &mut santa);
        grid.mark(santa);

        let Some(c) = chars.next() else {
            break;
        };

        move_house(c, &mut robo);
        grid.mark(robo);
    }

    grid.len().into()
}
