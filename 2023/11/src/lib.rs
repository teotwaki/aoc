use common::Answer;
use itertools::Itertools;

fn parse(s: &str) -> Vec<Vec<Option<()>>> {
    s.lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => None,
                    '#' => Some(()),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn empty_cols(row: &[Option<()>]) -> Vec<usize> {
    row.iter()
        .enumerate()
        .filter_map(|(i, x)| if x.is_none() { Some(i) } else { None })
        .collect()
}

fn calculate_sum(
    galaxies: &[(usize, usize)],
    vertical_expansions: &[usize],
    horizontal_expansions: &[usize],
    expansion_factor: usize,
) -> usize {
    galaxies
        .iter()
        .enumerate()
        .combinations(2)
        .map(|mut coords| {
            coords.sort_by(|a, b| a.1.cmp(b.1));

            let (x1, y1) = coords[0].1;
            let (x2, y2) = coords[1].1;

            let expansion_x = vertical_expansions
                .iter()
                .filter(|x| {
                    if x1 < x2 {
                        (x1..x2).contains(x)
                    } else {
                        (x2..x1).contains(x)
                    }
                })
                .count();

            let expansion_y = horizontal_expansions
                .iter()
                .filter(|y| {
                    if y1 < y2 {
                        (y1..y2).contains(y)
                    } else {
                        (y2..y1).contains(y)
                    }
                })
                .count();

            x1.abs_diff(*x2) + y1.abs_diff(*y2) + (expansion_x + expansion_y) * expansion_factor
        })
        .sum()
}

fn analyse_scan(scan: Vec<Vec<Option<()>>>) -> (Vec<usize>, Vec<usize>, Vec<(usize, usize)>) {
    let vertical_expansions: Vec<_> = scan
        .iter()
        .enumerate()
        .filter_map(|(i, r)| {
            if r.iter().all(|x| x.is_none()) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let horizontal_expansions: Vec<_> = scan.iter().fold(empty_cols(&scan[0]), |acc, r| {
        r.iter()
            .enumerate()
            .filter_map(|(i, x)| if x.is_none() { Some(i) } else { None })
            .filter(|x| acc.contains(x))
            .collect()
    });

    let galaxies: Vec<_> = scan
        .iter()
        .enumerate()
        .flat_map(|(x, r)| {
            r.iter()
                .enumerate()
                .filter_map(|(y, c)| if c.is_some() { Some((x, y)) } else { None })
                .collect::<Vec<_>>()
        })
        .collect();

    (vertical_expansions, horizontal_expansions, galaxies)
}

pub fn step1(s: &str) -> Answer {
    let scan = parse(s);
    let (vertical_expansions, horizontal_expansions, galaxies) = analyse_scan(scan);

    calculate_sum(&galaxies, &vertical_expansions, &horizontal_expansions, 1).into()
}

pub fn step2(s: &str) -> Answer {
    let scan = parse(s);
    let (vertical_expansions, horizontal_expansions, galaxies) = analyse_scan(scan);

    calculate_sum(
        &galaxies,
        &vertical_expansions,
        &horizontal_expansions,
        999_999,
    )
    .into()
}
