use std::iter::successors;

pub fn number_length(n: u64) -> usize {
    (n as f64).log10().floor() as usize + 1
}

pub fn number_length_successors(n: u64) -> usize {
    successors(Some(n), |&n| (n >= 10).then_some(n / 10)).count()
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
}
