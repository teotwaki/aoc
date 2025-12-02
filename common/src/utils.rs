use memoize::memoize;
use std::iter::successors;

pub fn number_length(n: u64) -> usize {
    (n as f64).log10().floor() as usize + 1
}

pub fn number_length_u128(n: u128) -> usize {
    (n as f64).log10().floor() as usize + 1
}

pub fn number_length_successors(n: u64) -> usize {
    successors(Some(n), |&n| (n >= 10).then_some(n / 10)).count()
}

pub fn concat_numbers(a: u64, b: u64) -> u64 {
    a * 10u64.pow(number_length(b) as u32) + b
}

pub fn concat_numbers_checked(a: u64, b: u64) -> Option<u64> {
    a.checked_mul(10u64.pow(number_length(b) as u32))
        .and_then(|a| a.checked_add(b))
}

pub fn concat_numbers_u128(a: u128, b: u128) -> u128 {
    a * 10u128.pow(number_length_u128(b) as u32) + b
}

pub fn concat_numbers_u128_checked(a: u128, b: u128) -> Option<u128> {
    a.checked_mul(10u128.pow(number_length_u128(b) as u32))
        .and_then(|a| a.checked_add(b))
}

#[memoize]
pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![1, n];

    let mut i = 2;

    while i * i <= n {
        if n.is_multiple_of(i) {
            factors.push(i);

            if i * i != n {
                factors.push(n / i);
            }
        }

        i += 1;
    }

    factors.sort();
    factors.dedup();

    factors
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn factors_finds_correct_factors() {
        assert_eq!(factors(1), vec![1]);
        assert_eq!(factors(6), vec![1, 2, 3, 6]);
        assert_eq!(factors(28), vec![1, 2, 4, 7, 14, 28]);
        assert_eq!(factors(49), vec![1, 7, 49]);
        assert_eq!(factors(100), vec![1, 2, 4, 5, 10, 20, 25, 50, 100]);
    }

    #[test]
    fn number_length_finds_correct_length() {
        assert_eq!(number_length(0), 1);
        assert_eq!(number_length(1000), 4);
        assert_eq!(number_length(1_000_000_000), 10);
        assert_eq!(number_length(10_000_000_000_000_000_000), 20);
        assert_eq!(number_length(u64::MAX), 20);
    }

    #[test]
    fn number_length_successors_finds_correct_length() {
        assert_eq!(number_length_successors(0), 1);
        assert_eq!(number_length_successors(1000), 4);
        assert_eq!(number_length_successors(1_000_000_000), 10);
        assert_eq!(number_length_successors(10_000_000_000_000_000_000), 20);
        assert_eq!(number_length_successors(u64::MAX), 20);
    }

    #[test]
    fn number_length_u128_handles_max_length() {
        assert_eq!(number_length_u128(u128::MAX), 39);
    }
}
