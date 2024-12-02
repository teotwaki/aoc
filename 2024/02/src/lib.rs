use common::Answer;

type IntType = u8;

fn parse(s: &str) -> Vec<Vec<IntType>> {
    s.lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<IntType>().unwrap())
                .collect()
        })
        .collect()
}

#[inline]
fn valid_report(report: &[IntType]) -> bool {
    report.windows(2).all(|w| {
        let diff = w[0].abs_diff(w[1]);
        diff > 0 && diff <= 3 && w[0] < w[1]
    }) || report.windows(2).all(|w| {
        let diff = w[0].abs_diff(w[1]);
        diff > 0 && diff <= 3 && w[0] > w[1]
    })
}

pub fn step1(s: &str) -> Answer {
    let reports = parse(s);

    reports.iter().filter(|r| valid_report(r)).count().into()
}

fn dampen(report: &[IntType]) -> Vec<Vec<IntType>> {
    (0..report.len())
        .map(|i| {
            let mut r = Vec::from(report);
            r.remove(i);

            r
        })
        .collect()
}

pub fn step2(s: &str) -> Answer {
    let reports = parse(s);

    let initial_valid = reports.iter().filter(|r| valid_report(r)).count();

    let dampened_valid = reports
        .iter()
        .filter(|r| !valid_report(r))
        .filter(|r| dampen(r).iter().any(|r| valid_report(r)))
        .count();

    (initial_valid + dampened_valid).into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn parse_extracts_correct_number_of_lines() {
        assert_eq!(parse(INPUT).len(), 6);
    }

    #[test]
    fn parse_extracts_correct_number_of_reports() {
        assert_eq!(parse(INPUT).first().unwrap().len(), 5);
    }

    #[test]
    fn step1_has_correct_answer() {
        assert_eq!(step1(INPUT), Answer::Unsigned(2));
    }

    #[test]
    fn step2_has_correct_answer() {
        assert_eq!(step2(INPUT), Answer::Unsigned(4));
    }
}
