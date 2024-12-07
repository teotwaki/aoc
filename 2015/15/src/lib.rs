use std::iter::repeat_n;

use common::Answer;
use itertools::Itertools;

type IntType = i64;

#[derive(Debug, Clone, Copy)]
struct Ingredient {
    capacity: IntType,
    durability: IntType,
    flavor: IntType,
    texture: IntType,
    calories: IntType,
}

impl Ingredient {
    fn mul(&self, amount: IntType) -> (IntType, IntType, IntType, IntType, IntType) {
        (
            self.capacity * amount,
            self.durability * amount,
            self.flavor * amount,
            self.texture * amount,
            self.calories * amount,
        )
    }
}

fn parse(s: &str) -> impl Iterator<Item = Ingredient> + '_ {
    s.lines().map(|l| {
        let mut parts = l.split_whitespace();
        let capacity = parts
            .nth(2)
            .and_then(|s| s.trim_matches(',').parse().ok())
            .unwrap();
        let durability = parts
            .nth(1)
            .and_then(|s| s.trim_matches(',').parse().ok())
            .unwrap();
        let flavor = parts
            .nth(1)
            .and_then(|s| s.trim_matches(',').parse().ok())
            .unwrap();
        let texture = parts
            .nth(1)
            .and_then(|s| s.trim_matches(',').parse().ok())
            .unwrap();
        let calories = parts.nth(1).and_then(|s| s.parse().ok()).unwrap();

        Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    })
}

fn find_recipes(ingredients: &[Ingredient]) -> impl Iterator<Item = (IntType, IntType)> + '_ {
    repeat_n(0..=100, ingredients.len())
        .multi_cartesian_product()
        .filter(|v| v.iter().sum::<IntType>() == 100)
        .map(|v| {
            v.iter()
                .zip(ingredients.iter())
                .map(|(&amount, ingredient)| ingredient.mul(amount))
                .fold(
                    (0, 0, 0, 0, 0),
                    |(ac, ad, af, at, acal), (c, d, f, t, cal)| {
                        (ac + c, ad + d, af + f, at + t, acal + cal)
                    },
                )
        })
        .map(|(c, d, f, t, cal)| {
            let c = if c < 0 { 0 } else { c };
            let d = if d < 0 { 0 } else { d };
            let f = if f < 0 { 0 } else { f };
            let t = if t < 0 { 0 } else { t };

            (c * d * f * t, cal)
        })
}

pub fn step1(s: &str) -> Answer {
    let ingredients = parse(s).collect::<Vec<_>>();
    find_recipes(&ingredients)
        .map(|(i, _)| i)
        .max()
        .unwrap()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let ingredients = parse(s).collect::<Vec<_>>();
    find_recipes(&ingredients)
        .filter(|&(_, cal)| cal == 500)
        .map(|(i, _)| i)
        .max()
        .unwrap()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#;

    #[test]
    fn step1_finds_correct_answer() {
        assert_eq!(step1(INPUT), Answer::Signed(62842880));
    }

    #[test]
    fn step2_finds_correct_answer() {
        assert_eq!(step2(INPUT), Answer::Signed(57600000));
    }
}
