use common::Answer;

type IntType = u16;

fn parse_area(s: &str) -> IntType {
    s.split('x')
        .map(|n| n.parse::<IntType>().unwrap())
        .product()
}

fn parse_gift_counts(s: &str) -> Vec<IntType> {
    s.split_whitespace()
        .map(|n| n.parse::<IntType>().unwrap())
        .collect()
}

fn parse_line(s: &str) -> (IntType, Vec<IntType>) {
    let mut parts = s.split(": ");

    let area = parts.next().map(parse_area).unwrap();
    let gift_counts = parts.next().map(parse_gift_counts).unwrap();

    (area, gift_counts)
}

fn parse_present_size(s: &str) -> IntType {
    let max_x = s.lines().skip(1).map(|l| l.len()).max().unwrap();
    let y = s.lines().skip(1).count();

    (max_x * y) as IntType
}

fn parse(s: &str) -> (Vec<IntType>, Vec<(IntType, Vec<IntType>)>) {
    let groups = s.split("\n\n").collect::<Vec<_>>();

    let gift_sizes = groups
        .iter()
        .take(groups.len() - 1)
        .map(|g| parse_present_size(g))
        .collect::<Vec<_>>();
    let areas = groups.last().unwrap().lines().map(parse_line).collect();

    (gift_sizes, areas)
}

pub fn step1(s: &str) -> Answer {
    let (gift_sizes, areas) = parse(s);

    areas
        .iter()
        .filter(|(area, gift_counts)| {
            let required_area = gift_counts
                .iter()
                .enumerate()
                .map(|(i, count)| gift_sizes[i] * count)
                .sum::<IntType>();

            *area >= required_area
        })
        .count()
        .into()
}

pub fn step2(_: &str) -> Answer {
    // No step 2 on the last day
    ().into()
}
