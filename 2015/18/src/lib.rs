use common::{Answer, BooleanGrid, Coordinates};

type IntType = i8;
type Lights = BooleanGrid<IntType>;
type Coords = Coordinates<IntType>;

fn parse(s: &str) -> Lights {
    let mut lights = Lights::new();

    s.lines().enumerate().for_each(|(y, l)| {
        l.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .for_each(|(x, _)| lights.mark((x as u8, y as u8).into()))
    });

    lights
}

fn update(lights: &mut Lights, broken: bool) {
    if broken {
        lights.mark((lights.width() - 1, lights.height() - 1).into());
        lights.mark((0, lights.height() - 1).into());
        lights.mark((lights.width() - 1, 0).into());
        lights.mark((0, 0).into());
    }

    let start_state = lights.clone();

    (0..start_state.width()).for_each(|x| {
        (0..start_state.height()).for_each(|y| {
            let xy = (x, y);
            if broken
                && (xy == (0, 0)
                    || xy == (lights.width() - 1, lights.height() - 1)
                    || xy == (0, lights.height() - 1)
                    || xy == (lights.height() - 1, 0))
            {
                return;
            }

            let pos = Coords::from((x, y));
            let light = lights.contains(&pos);

            let count = pos
                .neighbors_8()
                .into_iter()
                .filter(|pos| start_state.contains(pos))
                .count();

            if light && count != 2 && count != 3 {
                lights.remove(&pos);
            } else if !light && count == 3 {
                lights.mark(pos);
            }
        })
    });
}

fn animate(s: &str, iterations: usize, broken: bool) -> usize {
    let mut lights = parse(s);

    for _ in 0..iterations {
        update(&mut lights, broken);
    }

    lights.len()
}

pub fn step1(s: &str) -> Answer {
    animate(s, 100, false).into()
}

pub fn step2(s: &str) -> Answer {
    animate(s, 100, true).into()
}

#[cfg(test)]
mod test_2015_18 {
    use super::*;

    #[test]
    fn animate_working_screen_gets_correct_example_result() {
        let input = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#;
        assert_eq!(animate(input, 4, false), 4);
    }

    #[test]
    fn animate_broken_screen_gets_correct_example_result() {
        let input = r#"##.#.#
...##.
#....#
..#...
#.#..#
####.#"#;
        assert_eq!(animate(input, 5, true), 17);
    }
}
