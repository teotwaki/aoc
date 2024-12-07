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

#[cfg(test)]
mod test {
    use super::*;

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
