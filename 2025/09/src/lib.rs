use common::{Answer, Coordinates};
use itertools::Itertools;

type IntType = usize;
type Coords = Coordinates<IntType>;

fn parse(s: &str) -> impl Iterator<Item = Coords> {
    s.lines().map(|l| {
        let parts = l
            .split(',')
            .map(|i| i.parse::<IntType>().expect("Not a valid integer"))
            .collect::<Vec<_>>();

        Coords::new(parts[0], parts[1])
    })
}

fn rectangle_area(a: Coords, b: Coords) -> IntType {
    let x = a.x().abs_diff(b.x()) + 1;
    let y = a.y().abs_diff(b.y()) + 1;

    x * y
}

pub fn step1(s: &str) -> Answer {
    parse(s)
        .combinations(2)
        .map(|corners| rectangle_area(corners[0], corners[1]))
        .max()
        .expect("no maximum value found")
        .into()
}

fn edges(reds: &[Coords]) -> Vec<(Coords, Coords)> {
    let mut edges = reds.windows(2).map(|w| (w[0], w[1])).collect::<Vec<_>>();
    edges.push((reds[0], reds[reds.len() - 1]));

    edges
}

fn crosses_edges(rectangle: (Coords, Coords), edges: &[(Coords, Coords)]) -> bool {
    let min_x = rectangle.0.x().min(rectangle.1.x());
    let max_x = rectangle.0.x().max(rectangle.1.x());
    let min_y = rectangle.0.y().min(rectangle.1.y());
    let max_y = rectangle.0.y().max(rectangle.1.y());

    edges.iter().any(|edge| {
        let edge_min_x = edge.0.x().min(edge.1.x());
        let edge_max_x = edge.0.x().max(edge.1.x());
        let edge_min_y = edge.0.y().min(edge.1.y());
        let edge_max_y = edge.0.y().max(edge.1.y());

        min_x < edge_max_x && max_x > edge_min_x && min_y < edge_max_y && max_y > edge_min_y
    })
}

pub fn step2(s: &str) -> Answer {
    let reds = parse(s).collect::<Vec<_>>();
    let edges = edges(&reds);

    reds.iter()
        .combinations(2)
        .map(|corners| {
            (
                (corners[0], corners[1]),
                rectangle_area(*corners[0], *corners[1]),
            )
        })
        .sorted_unstable_by_key(|t| t.1)
        .rev()
        .find(|(r, _)| !crosses_edges((*r.0, *r.1), &edges))
        .map(|(_, area)| area)
        .unwrap()
        .into()
}

#[cfg(test)]
mod test_2025_09 {
    use super::*;

    const INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn area_calculates_correct_value() {
        let a = Coords::new(2, 5);
        let b = Coords::new(9, 7);

        assert_eq!(rectangle_area(a, b), 24);

        let a = Coords::new(7, 1);
        let b = Coords::new(11, 7);

        assert_eq!(rectangle_area(a, b), 35);

        let a = Coords::new(7, 3);
        let b = Coords::new(2, 3);

        assert_eq!(rectangle_area(a, b), 6);
    }

    #[test]
    fn parse_extracts_correct_number_of_lines() {
        assert_eq!(parse(INPUT).count(), 8);
    }

    #[test]
    fn step1_computes_expected_sample_result() {
        assert_eq!(step1(INPUT), Answer::Unsigned(50));
    }

    #[test]
    fn step2_computes_expected_sample_result() {
        assert_eq!(step2(INPUT), Answer::Unsigned(24));
    }
}
