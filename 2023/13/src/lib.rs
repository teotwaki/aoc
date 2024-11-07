use common::Answer;

#[derive(Debug)]
struct Pattern<'a> {
    rows: Vec<&'a str>,
}

impl<'a> From<&'a str> for Pattern<'a> {
    fn from(s: &'a str) -> Self {
        let rows = s.lines().collect();

        Pattern { rows }
    }
}

impl Pattern<'_> {
    fn horizontal_numbers(&self) -> Vec<u32> {
        self.rows
            .iter()
            .map(|row| {
                row.chars()
                    .enumerate()
                    .filter_map(|(i, c)| if c == '#' { Some(1 << i) } else { None })
                    .sum()
            })
            .collect()
    }

    fn vertical_numbers(&self) -> Vec<u32> {
        (0..self.rows[0].len())
            .map(|i| {
                self.rows
                    .iter()
                    .enumerate()
                    .map(|(j, row)| (j, row.chars().nth(i)))
                    .filter_map(|(j, c)| if c == Some('#') { Some(1 << j) } else { None })
                    .sum()
            })
            .collect()
    }

    fn summarize(&self) -> usize {
        let (v, _) = find_reflection(&self.vertical_numbers());
        let (h, _) = find_reflection(&self.horizontal_numbers());

        v + h * 100
    }

    fn summarize_smudges(&self) -> usize {
        let (_, v) = find_reflection(&self.vertical_numbers());
        let (_, h) = find_reflection(&self.horizontal_numbers());

        v + h * 100
    }
}

#[inline]
fn bit_diff(a: u32, b: u32) -> u32 {
    (a ^ b).count_ones()
}

fn find_reflection(values: &[u32]) -> (usize, usize) {
    values
        .windows(2)
        .enumerate()
        .filter_map(|(i, w)| {
            if bit_diff(w[0], w[1]) <= 1 {
                Some(i)
            } else {
                None
            }
        })
        .fold((0, 0), |acc, pos| {
            let mut min = pos;
            let mut max = pos + 1;
            let mut diff = 0;

            loop {
                diff += bit_diff(values[min], values[max]);

                if diff > 1 {
                    return acc;
                }

                if min > 0 && max < values.len() - 1 {
                    min -= 1;
                    max += 1;
                } else {
                    break;
                }
            }

            match diff {
                0 => (acc.0 + pos + 1, acc.1),
                1 => (acc.0, acc.1 + pos + 1),
                _ => acc,
            }
        })
}

fn sum_summary(s: &str, f: fn(&Pattern) -> usize) -> usize {
    s.split("\n\n").map(|s| f(&Pattern::from(s))).sum()
}

pub fn step1(s: &str) -> Answer {
    sum_summary(s, |p| p.summarize()).into()
}

pub fn step2(s: &str) -> Answer {
    sum_summary(s, |p| p.summarize_smudges()).into()
}
