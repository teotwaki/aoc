use common::Answer;
use rustc_hash::FxHashMap;

fn parse(s: &str) -> Vec<&str> {
    s.lines().collect()
}

fn get_move(from: char, to: char) -> &'static str {
    match (from, to) {
        ('0', '0') => "A",
        ('0', '1') => "^<A",
        ('0', '2') => "^A",
        ('0', '3') => "^>A",
        ('0', '4') => "^^<A",
        ('0', '5') => "^^A",
        ('0', '6') => "^^>A",
        ('0', '7') => "^^^<A",
        ('0', '8') => "^^^A",
        ('0', '9') => "^^^>A",
        ('0', 'A') => ">A",

        ('1', '0') => ">vA",
        ('1', '1') => "A",
        ('1', '2') => ">A",
        ('1', '3') => ">>A",
        ('1', '4') => "^A",
        ('1', '5') => "^>A",
        ('1', '6') => "^>>A",
        ('1', '7') => "^^A",
        ('1', '8') => "^^>A",
        ('1', '9') => "^^>>A",
        ('1', 'A') => ">>vA",

        ('2', '0') => "vA",
        ('2', '1') => "<A",
        ('2', '2') => "A",
        ('2', '3') => ">A",
        ('2', '4') => "<^A",
        ('2', '5') => "^A",
        ('2', '6') => "^>A",
        ('2', '7') => "<^^A",
        ('2', '8') => "^^A",
        ('2', '9') => "^^>A",
        ('2', 'A') => "v>A",

        ('3', '0') => "<vA",
        ('3', '1') => "<<A",
        ('3', '2') => "<A",
        ('3', '3') => "A",
        ('3', '4') => "<<^A",
        ('3', '5') => "<^A",
        ('3', '6') => "^A",
        ('3', '7') => "<<^^A",
        ('3', '8') => "<^^A",
        ('3', '9') => "^^A",
        ('3', 'A') => "vA",

        ('4', '0') => ">vvA",
        ('4', '1') => "vA",
        ('4', '2') => "v>A",
        ('4', '3') => "v>>A",
        ('4', '4') => "A",
        ('4', '5') => ">A",
        ('4', '6') => ">>A",
        ('4', '7') => "^A",
        ('4', '8') => "^>A",
        ('4', '9') => "^>>A",
        ('4', 'A') => ">>vvA",

        ('5', '0') => "vvA",
        ('5', '1') => "<vA",
        ('5', '2') => "vA",
        ('5', '3') => "v>A",
        ('5', '4') => "<A",
        ('5', '5') => "A",
        ('5', '6') => ">A",
        ('5', '7') => "<^A",
        ('5', '8') => "^A",
        ('5', '9') => "^>A",
        ('5', 'A') => "vv>A",

        ('6', '0') => "<vvA",
        ('6', '1') => "<<vA",
        ('6', '2') => "<vA",
        ('6', '3') => "vA",
        ('6', '4') => "<<A",
        ('6', '5') => "<A",
        ('6', '6') => "A",
        ('6', '7') => "<<^A",
        ('6', '8') => "<^A",
        ('6', '9') => "^A",
        ('6', 'A') => "vvA",

        ('7', '0') => ">vvvA",
        ('7', '1') => "vvA",
        ('7', '2') => "vv>A",
        ('7', '3') => "vv>>A",
        ('7', '4') => "vA",
        ('7', '5') => "v>A",
        ('7', '6') => "v>>A",
        ('7', '7') => "A",
        ('7', '8') => ">A",
        ('7', '9') => ">>A",
        ('7', 'A') => ">>vvvA",

        ('8', '0') => "vvvA",
        ('8', '1') => "<vvA",
        ('8', '2') => "vvA",
        ('8', '3') => "vv>A",
        ('8', '4') => "<vA",
        ('8', '5') => "vA",
        ('8', '6') => "v>A",
        ('8', '7') => "<A",
        ('8', '8') => "A",
        ('8', '9') => ">A",
        ('8', 'A') => "vvv>A",

        ('9', '0') => "<vvvA",
        ('9', '1') => "<<vvA",
        ('9', '2') => "<vvA",
        ('9', '3') => "vvA",
        ('9', '4') => "<<vA",
        ('9', '5') => "<vA",
        ('9', '6') => "vA",
        ('9', '7') => "<<A",
        ('9', '8') => "<A",
        ('9', '9') => "A",
        ('9', 'A') => "vvvA",

        ('A', '0') => "<A",
        ('A', '1') => "^<<A",
        ('A', '2') => "<^A",
        ('A', '3') => "^A",
        ('A', '4') => "^^<<A",
        ('A', '5') => "<^^A",
        ('A', '6') => "^^A",
        ('A', '7') => "^^^<<A",
        ('A', '8') => "<^^^A",
        ('A', '9') => "^^^A",
        ('A', 'A') => "A",
        ('A', '^') => "<A",
        ('A', '>') => "vA",
        ('A', 'v') => "<vA",
        ('A', '<') => "v<<A",

        ('^', '^') => "A",
        ('^', '>') => "v>A",
        ('^', 'v') => "vA",
        ('^', '<') => "v<A",
        ('^', 'A') => ">A",

        ('>', '>') => "A",
        ('>', 'v') => "<A",
        ('>', '<') => "<<A",
        ('>', '^') => "<^A",
        ('>', 'A') => "^A",

        ('v', 'v') => "A",
        ('v', '<') => "<A",
        ('v', '^') => "^A",
        ('v', '>') => ">A",
        ('v', 'A') => "^>A",

        ('<', '<') => "A",
        ('<', '^') => ">^A",
        ('<', '>') => ">>A",
        ('<', 'v') => ">A",
        ('<', 'A') => ">>^A",

        _ => unreachable!(),
    }
}

fn transpose<'a>(code: &'a str, n: usize, cache: &mut FxHashMap<(&'a str, usize), usize>) -> usize {
    if let Some(&len) = cache.get(&(code, n)) {
        len
    } else if n == 0 {
        code.len()
    } else {
        let mut from = 'A';

        let len = code
            .chars()
            .map(|to| {
                let m = get_move(from, to);
                from = to;

                transpose(m, n - 1, cache)
            })
            .sum();

        cache.insert((code, n), len);

        len
    }
}

fn complexity<'a>(
    code: &'a str,
    n: usize,
    cache: &mut FxHashMap<(&'a str, usize), usize>,
) -> usize {
    let numeric_code = code[..3].parse::<usize>().unwrap();
    let instruction_length = transpose(code, n, cache);

    instruction_length * numeric_code
}

pub fn step1(s: &str) -> Answer {
    let mut cache = FxHashMap::default();

    parse(s)
        .into_iter()
        .map(|code| complexity(code, 3, &mut cache))
        .sum::<usize>()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let mut cache = FxHashMap::default();

    parse(s)
        .into_iter()
        .map(|code| complexity(code, 26, &mut cache))
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"029A
980A
179A
456A
379A"#;

    #[test]
    fn step1_finds_example_answer() {
        assert_eq!(step1(INPUT), Answer::Unsigned(126384));
    }

    #[test]
    fn complexity_finds_29_code_complexity() {
        let mut cache = FxHashMap::default();
        assert_eq!(complexity("029A", 3, &mut cache), 68 * 29);
    }

    #[test]
    fn complexity_finds_980_code_complexity() {
        let mut cache = FxHashMap::default();
        assert_eq!(complexity("980A", 3, &mut cache), 60 * 980);
    }

    #[test]
    fn complexity_finds_179_code_complexity() {
        let mut cache = FxHashMap::default();
        assert_eq!(complexity("179A", 3, &mut cache), 68 * 179);
    }

    #[test]
    fn complexity_finds_456_code_complexity() {
        let mut cache = FxHashMap::default();
        assert_eq!(complexity("456A", 3, &mut cache), 64 * 456);
    }

    #[test]
    fn complexity_finds_379_code_complexity() {
        let mut cache = FxHashMap::default();
        assert_eq!(complexity("379A", 3, &mut cache), 64 * 379);
    }

    #[test]
    fn transpose_character_is_fast_enough() {
        let mut cache = FxHashMap::default();
        assert_eq!(transpose("<", 26, &mut cache), 30331287706);
    }
}
