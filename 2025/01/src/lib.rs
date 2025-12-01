use common::Answer;

type IntType = i16;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Rotation {
    Left(IntType),
    Right(IntType),
}

fn parse(s: &str) -> Vec<Rotation> {
    s.lines()
        .map(|l| {
            let dir = l.chars().next().unwrap();
            let count = l[1..].parse::<IntType>().unwrap();

            match dir {
                'L' => Rotation::Left(count),
                'R' => Rotation::Right(count),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Dial {
    value: IntType,
    ended_on_zero: usize,
    passed_zero_count: usize,
}

impl Dial {
    pub fn new(v: IntType) -> Self {
        Self {
            value: v % 100,
            ended_on_zero: 0,
            passed_zero_count: 0,
        }
    }

    fn update_ended_on_zero(&mut self) {
        if self.value == 0 {
            self.ended_on_zero += 1;
        }
    }

    pub fn turn_left_simple(&mut self, count: IntType) {
        self.value = (self.value - count) % 100;
        self.update_ended_on_zero();
    }

    pub fn turn_right_simple(&mut self, count: IntType) {
        self.value = (self.value + count) % 100;
        self.update_ended_on_zero();
    }

    pub fn turn_left_4b(&mut self, count: IntType) {
        let remainder = if self.value == 0 { 100 } else { self.value };

        if count >= remainder {
            self.passed_zero_count += 1 + ((count - remainder) / 100) as usize;
        }

        self.value = ((self.value - count) % 100 + 100) % 100;
    }

    pub fn turn_right_4b(&mut self, count: IntType) {
        self.passed_zero_count += ((self.value + count) / 100) as usize;
        self.value = ((self.value + count) % 100 + 100) % 100;
    }
}

pub fn step1(s: &str) -> Answer {
    let mut dial = Dial::new(50);

    parse(s).iter().for_each(|rotation| match rotation {
        Rotation::Left(count) => dial.turn_left_simple(*count),
        Rotation::Right(count) => dial.turn_right_simple(*count),
    });

    Answer::Unsigned(dial.ended_on_zero as u64)
}

pub fn step2(s: &str) -> Answer {
    let mut dial = Dial::new(50);

    parse(s).iter().for_each(|rotation| match rotation {
        Rotation::Left(count) => dial.turn_left_4b(*count),
        Rotation::Right(count) => dial.turn_right_4b(*count),
    });

    Answer::Unsigned(dial.passed_zero_count as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "L18\nR5\nR20";

    #[test]
    fn parse_extracts_correct_number_of_lines() {
        assert_eq!(parse(INPUT).len(), 3);
    }

    #[test]
    fn parse_extracts_directions_correctly() {
        let rotations = parse(INPUT);

        assert_eq!(rotations[0], Rotation::Left(18));
        assert_eq!(rotations[1], Rotation::Right(5));
        assert_eq!(rotations[2], Rotation::Right(20));
    }

    #[test]
    fn turn_left_decrements_value() {
        let mut dial = Dial::new(50);
        dial.turn_left_simple(30);
        assert_eq!(dial.value, 20);
    }

    #[test]
    fn turn_right_increments_value() {
        let mut dial = Dial::new(50);
        dial.turn_right_simple(30);
        assert_eq!(dial.value, 80);
    }

    #[test]
    fn dial_handles_zero() {
        let mut dial = Dial::new(10);
        dial.turn_left_simple(10);
        assert_eq!(dial.value, 0);

        dial.turn_left_simple(1);
        assert_eq!(dial.value, 99);

        dial.turn_right_simple(2);
        assert_eq!(dial.value, 1);
    }

    const SAMPLE_INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn step1_works_on_sample_input() {
        let answer = step1(SAMPLE_INPUT);
        assert_eq!(answer, Answer::Unsigned(3));
    }

    #[test]
    fn step2_works_on_sample_input() {
        let answer = step2(SAMPLE_INPUT);
        assert_eq!(answer, Answer::Unsigned(6));
    }
}
