use std::collections::{HashMap, HashSet};

use common::Answer;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Token<'a> {
    Molecule(&'a str),
    Noise(&'a str),
}

impl<'a> Token<'a> {
    fn as_str(&self) -> &'a str {
        use Token::*;

        match self {
            Molecule(s) => s,
            Noise(s) => s,
        }
    }
}

fn parse(s: &str) -> (HashMap<&str, Vec<&str>>, Vec<Token>) {
    let mut parts = s.split("\n\n");
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    parts.next().unwrap().lines().for_each(|l| {
        let mut parts = l.split(" => ");
        map.entry(parts.next().unwrap())
            .or_default()
            .push(parts.next().unwrap());
    });

    let molecules = map.keys().copied().collect::<Vec<_>>();
    let mut calibration = parts.next().unwrap().trim_end();

    let mut tokens = vec![];

    'outer: while !calibration.is_empty() {
        for mol in &molecules {
            if calibration.starts_with(mol) {
                tokens.push(Token::Molecule(mol));
                calibration = &calibration[mol.len()..];
                continue 'outer;
            }
        }

        tokens.push(Token::Noise(&calibration[..1]));
        calibration = &calibration[1..];
    }

    (map, tokens)
}

fn generate_molecules(tokens: &[Token], needle: &str, replacement: &str) -> Vec<String> {
    let mut v = vec![];

    for i in 0..tokens.len() {
        v.push(
            tokens
                .iter()
                .enumerate()
                .map(|(j, token)| {
                    if i == j && *token == Token::Molecule(needle) {
                        replacement
                    } else {
                        token.as_str()
                    }
                })
                .collect::<String>(),
        );
    }

    v
}

pub fn step1(s: &str) -> Answer {
    let (recipes, tokens) = parse(s);
    let original = tokens.iter().map(|t| t.as_str()).collect::<String>();

    let molecules = recipes
        .iter()
        .flat_map(|(tok, replacements)| {
            replacements
                .iter()
                .flat_map(|rep| generate_molecules(&tokens, tok, rep))
        })
        .filter(|s| s != &original)
        .collect::<HashSet<_>>();

    molecules.len().into()
}

pub fn step2(_: &str) -> Answer {
    ().into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn step1_finds_correct_example_answer() {
        let input = r#"H => HO
H => OH
O => HH

HOH"#;

        assert_eq!(step1(input), Answer::Unsigned(4));
    }
}
