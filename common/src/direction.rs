#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    #[inline]
    #[must_use]
    pub fn clockwise(&self) -> Self {
        use Direction::*;

        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    #[inline]
    pub fn turn_clockwise(&mut self) {
        *self = self.clockwise();
    }

    #[inline]
    #[must_use]
    pub fn counterclockwise(&self) -> Self {
        use Direction::*;

        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    #[inline]
    pub fn turn_counterclockwise(&mut self) {
        *self = self.counterclockwise();
    }
}
