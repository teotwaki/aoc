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
