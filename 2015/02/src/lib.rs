use common::Answer;

type IntType = u32;

fn parse(s: &str) -> impl Iterator<Item = Vec<IntType>> + '_ {
    s.lines().map(|l| {
        l.split('x')
            .map(|i| i.parse::<IntType>().unwrap())
            .collect()
    })
}

fn wrapping_requirement(dimensions: &[IntType]) -> IntType {
    let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);
    let min = (l * w).min(w * h).min(h * l);

    2 * l * w + 2 * w * h + 2 * h * l + min
}

pub fn step1(s: &str) -> Answer {
    parse(s)
        .map(|d| wrapping_requirement(&d))
        .sum::<IntType>()
        .into()
}

fn ribbon_requirement(dimensions: &[IntType]) -> IntType {
    let mut dimensions = Vec::from(dimensions);
    let ribbon = dimensions.iter().product::<IntType>();

    dimensions.sort();

    2 * dimensions[0] + 2 * dimensions[1] + ribbon
}

pub fn step2(s: &str) -> Answer {
    parse(s)
        .map(|d| ribbon_requirement(&d))
        .sum::<IntType>()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wrapping_requirement_calculates_correct_value() {
        assert_eq!(wrapping_requirement(&[2, 3, 4]), 58);
    }

    #[test]
    fn ribbon_requirement_calculates_correct_value() {
        assert_eq!(ribbon_requirement(&[2, 3, 4]), 34);
    }
}
