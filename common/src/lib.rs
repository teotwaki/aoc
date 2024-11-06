use std::fmt::Display;

pub enum Answer {
    Signed(i64),
    Unsigned(u64),
    Text(String),
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Answer::*;

        match self {
            Signed(i) => i.fmt(f),
            Unsigned(u) => u.fmt(f),
            Text(s) => s.fmt(f),
        }
    }
}

impl From<usize> for Answer {
    fn from(value: usize) -> Self {
        Answer::Unsigned(value as u64)
    }
}

impl From<u32> for Answer {
    fn from(value: u32) -> Self {
        Answer::Unsigned(value as u64)
    }
}

impl From<u16> for Answer {
    fn from(value: u16) -> Self {
        Answer::Unsigned(value as u64)
    }
}

impl From<u8> for Answer {
    fn from(value: u8) -> Self {
        Answer::Unsigned(value as u64)
    }
}

impl From<i32> for Answer {
    fn from(value: i32) -> Self {
        Answer::Signed(value as i64)
    }
}

impl From<i16> for Answer {
    fn from(value: i16) -> Self {
        Answer::Signed(value as i64)
    }
}

impl From<i8> for Answer {
    fn from(value: i8) -> Self {
        Answer::Signed(value as i64)
    }
}
