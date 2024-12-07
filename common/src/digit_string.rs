use crate::utils::{
    concat_numbers_checked, concat_numbers_u128_checked, number_length, number_length_u128,
};
use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct DigitString {
    buf: Vec<u64>,
}

impl DigitString {
    pub fn new() -> Self {
        Self { buf: vec![] }
    }

    pub fn push(&mut self, digits: u64) {
        if self.buf.is_empty() {
            self.buf.push(0);
        }

        let current = *self.buf.last().unwrap_or(&0);

        if let Some(val) = concat_numbers_checked(current, digits) {
            *self.buf.last_mut().unwrap() = val;
        } else {
            self.buf.push(digits);
        }
    }

    pub fn len(&self) -> usize {
        self.buf.iter().map(|&n| number_length(n)).sum()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}

impl Display for DigitString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.buf.iter().try_for_each(|n| n.fmt(f))
    }
}

#[derive(Debug, Clone, Default)]
pub struct DigitStringU128 {
    buf: Vec<u128>,
}

impl DigitStringU128 {
    pub fn new() -> Self {
        Self { buf: vec![] }
    }

    pub fn push(&mut self, digits: u128) {
        if self.buf.is_empty() {
            self.buf.push(0);
        }

        let current = *self.buf.last().unwrap_or(&0);

        if let Some(val) = concat_numbers_u128_checked(current, digits) {
            *self.buf.last_mut().unwrap() = val;
        } else {
            self.buf.push(digits);
        }
    }

    pub fn len(&self) -> usize {
        self.buf.iter().map(|&n| number_length_u128(n)).sum()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}

impl Display for DigitStringU128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.buf.iter().try_for_each(|n| n.fmt(f))
    }
}
