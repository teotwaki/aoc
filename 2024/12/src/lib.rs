use common::{Answer, Coordinates, Grid};
use rayon::prelude::*;
use rustc_hash::FxHashSet;

type IntType = i16;
type Garden = Grid<IntType, char>;
type Coords = Coordinates<IntType>;

fn parse(s: &str) -> Garden {
    let mut garden = Garden::new();

    s.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .for_each(|(x, c)| garden.store((x, y).into(), c))
    });

    garden
}

fn find_plot_groups(garden: &Garden) -> Vec<FxHashSet<Coords>> {
    let mut assigned_plots: FxHashSet<Coords> = FxHashSet::default();
    let mut plot_groups = vec![];

    for (plot, val) in garden.iter() {
        if !assigned_plots.contains(plot) {
            let group = garden.flood(*plot, |_, b| val == b);
            assigned_plots.extend(group.iter());
            plot_groups.push(group);
        }
    }

    plot_groups
}

fn fence_length(garden: &Garden, plot: Coords) -> usize {
    let crop_type = garden.get(&plot);
    plot.neighbors()
        .iter()
        .filter(|pos| garden.get(pos) != crop_type)
        .count()
}

pub fn step1(s: &str) -> Answer {
    let garden = parse(s);
    find_plot_groups(&garden)
        .par_iter()
        .map(|plot_group| {
            (
                plot_group.len(),
                plot_group
                    .iter()
                    .map(|&plot| fence_length(&garden, plot))
                    .sum::<usize>(),
            )
        })
        .map(|(area, fence_length)| area * fence_length)
        .sum::<usize>()
        .into()
}

fn count_sides(plot_group: &FxHashSet<Coords>) -> usize {
    plot_group
        .iter()
        .map(|plot| {
            let non_neighbors = plot
                .neighbors()
                .into_iter()
                .filter(|p| !plot_group.contains(p))
                .collect::<Vec<_>>();

            let corners = match non_neighbors.len() {
                4 => 4,
                3 => 2,
                2 if non_neighbors[0].x() != non_neighbors[1].x()
                    && non_neighbors[0].y() != non_neighbors[1].y() =>
                {
                    1
                }
                _ => 0,
            };

            let checks = [
                (plot.left(), plot.down(), plot.southwest()),
                (plot.up(), plot.left(), plot.northwest()),
                (plot.right(), plot.up(), plot.northeast()),
                (plot.down(), plot.right(), plot.southeast()),
            ];

            let inner_corners = checks
                .iter()
                .filter(|(a, b, diag)| {
                    plot_group.contains(a) && plot_group.contains(b) && !plot_group.contains(diag)
                })
                .count();

            corners + inner_corners
        })
        .sum()
}

pub fn step2(s: &str) -> Answer {
    let garden = parse(s);
    find_plot_groups(&garden)
        .par_iter()
        .map(|plot_group| (plot_group.len(), count_sides(plot_group)))
        .map(|(area, fence_sides)| area * fence_sides)
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod test_2024_12 {
    use super::*;

    const INPUT: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    #[test]
    fn step1_find_correct_answer_example() {
        assert_eq!(step1(INPUT), Answer::Unsigned(1930));
    }

    #[test]
    fn step2_find_correct_answer_example() {
        assert_eq!(step2(INPUT), Answer::Unsigned(1206));
    }
}
