use num_traits::{self, ToPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position<T>(T, T);

impl<T> Position<T>
where
    T: Copy,
{
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    #[inline]
    pub fn x(&self) -> T {
        self.0
    }

    #[inline]
    pub fn y(&self) -> T {
        self.1
    }
}

impl<T, U> From<(U, U)> for Position<T>
where
    T: num_traits::NumCast,
    U: ToPrimitive,
{
    fn from(value: (U, U)) -> Self {
        Self(T::from(value.0).unwrap(), T::from(value.1).unwrap())
    }
}
