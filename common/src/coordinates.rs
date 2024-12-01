use num_traits::{Float, NumCast, PrimInt, ToPrimitive};
use std::ops::{AddAssign, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates<T> {
    x: T,
    y: T,
}

impl<T> Coordinates<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Coordinates<T>
where
    T: Copy,
{
    #[inline]
    pub fn x(&self) -> T {
        self.x
    }

    #[inline]
    pub fn y(&self) -> T {
        self.y
    }
}

impl<T> Coordinates<T>
where
    T: AddAssign + PrimInt,
{
    #[inline]
    pub fn down(&mut self) {
        self.y += T::one();
    }

    #[inline]
    pub fn right(&mut self) {
        self.x += T::one();
    }
}

impl<T> Coordinates<T>
where
    T: SubAssign + PrimInt,
{
    #[inline]
    pub fn up(&mut self) {
        self.y -= T::one();
    }

    #[inline]
    pub fn left(&mut self) {
        self.x -= T::one();
    }
}

impl<T, U, V> From<(U, V)> for Coordinates<T>
where
    T: NumCast,
    U: ToPrimitive,
    V: ToPrimitive,
{
    fn from(value: (U, V)) -> Self {
        Self::new(T::from(value.0).unwrap(), T::from(value.1).unwrap())
    }
}

impl<T> Coordinates<T>
where
    T: Copy + PrimInt,
{
    pub fn to_float<F: Float>(&self) -> Coordinates<F> {
        let x = F::from(self.x).unwrap();
        let y = F::from(self.y).unwrap();

        Coordinates::new(x, y)
    }
}

impl<F> Coordinates<F>
where
    F: Copy + Float,
{
    pub fn to_int<T: PrimInt>(&self) -> Coordinates<T> {
        let x = T::from(self.x).unwrap();
        let y = T::from(self.y).unwrap();

        Coordinates::new(x, y)
    }
}
