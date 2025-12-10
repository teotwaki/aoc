use common::Answer;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

type IntType = u64;

fn parse(s: &str) -> Vec<IntType> {
    s.lines().map(|l| l.parse::<IntType>().unwrap()).collect()
}

fn update_secret(mut i: IntType) -> IntType {
    let m = 1 << 24;

    i = ((i << 6) ^ i) % m;
    i = ((i >> 5) ^ i) % m;
    ((i << 11) ^ i) % m
}

fn nth_secret_number(i: IntType, n: usize) -> IntType {
    (0..n).fold(i, |i, _| update_secret(i))
}

fn seller_prices(mut i: IntType, n: usize) -> Vec<i8> {
    let first_price = (i % 10) as i8;
    let mut prices = (0..n)
        .map(|_| {
            i = update_secret(i);
            (i % 10) as i8
        })
        .collect::<Vec<_>>();

    prices.insert(0, first_price);

    prices
}

fn seller_changes(prices: &[i8]) -> Vec<i8> {
    prices.windows(2).map(|w| w[1] - w[0]).collect()
}

pub fn step1(s: &str) -> Answer {
    parse(s)
        .into_par_iter()
        .map(|i| nth_secret_number(i, 2000))
        .sum::<IntType>()
        .into()
}

pub fn step2(s: &str) -> Answer {
    let mut pattern_values: FxHashMap<(i8, i8, i8, i8), i16> = FxHashMap::default();
    let vendor_prices = parse(s)
        .into_par_iter()
        .map(|i| seller_prices(i, 2000))
        .collect::<Vec<_>>();
    let mut vendor_patterns = vec![FxHashSet::default(); vendor_prices.len()];

    vendor_prices
        .iter()
        .enumerate()
        .for_each(|(vendor, prices)| {
            seller_changes(prices)
                .windows(4)
                .enumerate()
                .for_each(|(i, changes)| {
                    let pattern = (changes[0], changes[1], changes[2], changes[3]);

                    if vendor_patterns[vendor].insert(pattern) {
                        *pattern_values.entry(pattern).or_default() +=
                            vendor_prices[vendor][i + 4] as i16;
                    }
                });
        });

    pattern_values.values().max().copied().unwrap().into()
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(
        input1 = { 123, 1, 10, 100, 2024 },
        input2 = { 10, 2000, 2000, 2000, 2000 },
        result = { 5908254, 8685429, 4700978, 15273692, 8667524 },
    )]
    fn verify_math(input1: IntType, input2: usize, result: IntType) {
        assert_eq!(nth_secret_number(input1, input2), result);
    }

    #[test]
    fn step2_finds_correct_pattern_and_bananas() {
        let input = "1
2
3
2024";

        assert_eq!(step2(input), Answer::Signed(23));
    }
}
