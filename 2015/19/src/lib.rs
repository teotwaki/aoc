use rayon::prelude::*;
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

fn tokenize<'a>(mut data: &'a str, tokens: &[&'a str]) -> Vec<Token<'a>> {
    let mut tokenized = vec![];

    'outer: while !data.is_empty() {
        for token in tokens {
            if data.starts_with(token) {
                tokenized.push(Token::Molecule(token));
                data = &data[token.len()..];
                continue 'outer;
            }
        }

        tokenized.push(Token::Noise(&data[..1]));
        data = &data[1..];
    }

    tokenized
}

fn parse(s: &str) -> (HashMap<&str, Vec<&str>>, &str) {
    let mut parts = s.split("\n\n");
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    parts.next().unwrap().lines().for_each(|l| {
        let mut parts = l.split(" => ");
        map.entry(parts.next().unwrap())
            .or_default()
            .push(parts.next().unwrap());
    });

    (map, parts.next().unwrap().trim_end())
}

fn generate_molecules(tokens: &[Token], needle: &str, replacement: &str) -> Vec<String> {
    (0..tokens.len())
        .map(|i| {
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
                .collect::<String>()
        })
        .collect()
}

pub fn step1(s: &str) -> Answer {
    let (recipes, original) = parse(s);
    let molecules = recipes.keys().copied().collect::<Vec<_>>();
    let medecine = tokenize(original, &molecules);

    let molecules = recipes
        .par_iter()
        .flat_map(|(tok, replacements)| {
            replacements
                .par_iter()
                .flat_map(|rep| generate_molecules(&medecine, tok, rep))
        })
        .filter(|s| s != original)
        .collect::<HashSet<_>>();

    molecules.len().into()
}

fn to_elements(mut s: &str) -> Vec<&str> {
    let mut elements = vec![];

    while !s.is_empty() {
        if let Some(second) = s.chars().nth(1)
            && second.is_lowercase()
        {
            elements.push(&s[..2]);
            s = &s[2..];

            continue;
        }

        elements.push(&s[..1]);
        s = &s[1..];
    }

    elements
}

pub fn step2(s: &str) -> Answer {
    let medecine = s.trim_end().split("\n\n").nth(1).unwrap();
    let elements = to_elements(medecine);
    let paren_count = elements.iter().filter(|&&s| s == "Rn" || s == "Ar").count();
    let comma_count = elements.iter().filter(|&&s| s == "Y").count();

    (elements.len() - paren_count - 2 * comma_count - 1).into()
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
