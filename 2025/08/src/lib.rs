use common::Answer;
use itertools::Itertools;
use rustc_hash::FxHashSet;

type IntType = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
struct Xyz {
    x: IntType,
    y: IntType,
    z: IntType,
}

fn distance(a: &Xyz, b: &Xyz) -> u64 {
    let dx = a.x as i64 - b.x as i64;
    let dy = a.y as i64 - b.y as i64;
    let dz = a.z as i64 - b.z as i64;

    (dx * dx + dy * dy + dz * dz) as u64
}

fn parse(s: &str) -> Vec<Xyz> {
    s.lines()
        .map(|l| {
            let mut points = l.split(',').map(|s| s.parse::<IntType>().unwrap());

            Xyz {
                x: points.next().expect("Expected data at beginning of line"),
                y: points.next().expect("Expected second element on line"),
                z: points.next().expect("Expected third element on line"),
            }
        })
        .collect()
}

fn measure_distances(boxes: &[Xyz]) -> Vec<(u64, usize, usize)> {
    (0..boxes.len())
        .flat_map(move |i| {
            ((i + 1)..boxes.len()).map(move |j| (distance(&boxes[i], &boxes[j]), i, j))
        })
        .sorted_by_key(|d| d.0)
        .collect()
}

fn merge_circuits(circuits: &mut Vec<FxHashSet<Xyz>>, a: Xyz, b: Xyz) {
    let idx_a = circuits
        .iter()
        .enumerate()
        .find(|(_, c)| c.contains(&a))
        .map(|(i, _)| i);

    let idx_b = circuits
        .iter()
        .enumerate()
        .find(|(_, c)| c.contains(&b))
        .map(|(i, _)| i);

    match (idx_a, idx_b) {
        (Some(idx_a), Some(idx_b)) if idx_a != idx_b => {
            let src = circuits[idx_b].clone();
            let dest = circuits.get_mut(idx_a).unwrap();
            src.iter().for_each(|node| {
                dest.insert(*node);
            });
            circuits.remove(idx_b);
        }
        (None, Some(idx)) => {
            circuits[idx].insert(a);
        }
        (Some(idx), None) => {
            circuits[idx].insert(b);
        }
        (None, None) => circuits.push([a, b].into_iter().collect()),
        _ => {}
    }
}

fn connect_circuits(s: &str, n: usize) -> Answer {
    let boxes = parse(s);
    let distances = measure_distances(&boxes);

    let mut circuits: Vec<FxHashSet<Xyz>> = vec![];

    distances.iter().take(n).for_each(|(_, i, j)| {
        let a = boxes[*i];
        let b = boxes[*j];

        merge_circuits(&mut circuits, a, b);
    });

    circuits
        .iter()
        .map(|c| c.len())
        .sorted_unstable()
        .rev()
        .take(3)
        .product::<usize>()
        .into()
}

pub fn step1(s: &str) -> Answer {
    connect_circuits(s, 1000)
}

pub fn step2(s: &str) -> Answer {
    let boxes = parse(s);
    let distances = measure_distances(&boxes);

    let mut circuits: Vec<FxHashSet<Xyz>> = vec![];

    let mut a = boxes[0];
    let mut b = boxes[0];

    for (_, i, j) in distances {
        a = boxes[i];
        b = boxes[j];

        merge_circuits(&mut circuits, a, b);

        if circuits[0].len() == boxes.len() {
            break;
        }
    }

    (a.x * b.x).into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn parse_extracts_correct_number_of_lines() {
        assert_eq!(parse(INPUT).len(), 20);
    }

    #[test]
    fn connect_circuits_computes_expected_sample_result() {
        assert_eq!(connect_circuits(INPUT, 10), Answer::Unsigned(40));
    }

    #[test]
    fn step2_computes_expected_sample_result() {
        assert_eq!(step2(INPUT), Answer::Unsigned(25272));
    }
}
