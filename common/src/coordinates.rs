use crate::Direction;
use num_traits::{Float, NumCast, PrimInt, ToPrimitive};
use std::ops::{Add, AddAssign, Range, RangeInclusive, Sub, SubAssign};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
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
    Range<T>: DoubleEndedIterator<Item = T>,
    T: Copy,
{
    pub fn range(&self, rhs: Self) -> impl DoubleEndedIterator<Item = Self> + '_ {
        (self.x..rhs.x).flat_map(move |x| (self.y..rhs.y).map(move |y| Coordinates::new(x, y)))
    }
}

impl<T> Coordinates<T>
where
    RangeInclusive<T>: DoubleEndedIterator<Item = T>,
    T: Copy,
{
    pub fn range_inclusive(&self, rhs: Self) -> impl DoubleEndedIterator<Item = Self> + '_ {
        (self.x..=rhs.x).flat_map(move |x| (self.y..=rhs.y).map(move |y| Coordinates::new(x, y)))
    }
}

impl<T> Coordinates<T>
where
    T: AddAssign + SubAssign + PrimInt,
{
    #[inline]
    pub fn move_up(&mut self) -> &mut Self {
        self.y -= T::one();

        self
    }

    #[inline]
    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - T::one(),
        }
    }

    #[inline]
    pub fn move_down(&mut self) -> &mut Self {
        self.y += T::one();

        self
    }

    #[inline]
    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + T::one(),
        }
    }

    #[inline]
    pub fn move_left(&mut self) -> &mut Self {
        self.x -= T::one();

        self
    }

    #[inline]
    pub fn left(&self) -> Self {
        Self {
            x: self.x - T::one(),
            y: self.y,
        }
    }

    #[inline]
    pub fn move_right(&mut self) -> &mut Self {
        self.x += T::one();

        self
    }

    #[inline]
    pub fn right(&self) -> Self {
        Self {
            x: self.x + T::one(),
            y: self.y,
        }
    }

    #[inline]
    pub fn move_northwest(&mut self) -> &mut Self {
        self.move_up().move_left();

        self
    }

    #[inline]
    pub fn northwest(&self) -> Self {
        Self {
            x: self.x - T::one(),
            y: self.y - T::one(),
        }
    }

    #[inline]
    pub fn move_northeast(&mut self) -> &mut Self {
        self.move_up().move_right();

        self
    }

    #[inline]
    pub fn northeast(&self) -> Self {
        Self {
            x: self.x + T::one(),
            y: self.y - T::one(),
        }
    }

    #[inline]
    pub fn move_southeast(&mut self) -> &mut Self {
        self.move_down().move_right();

        self
    }

    #[inline]
    pub fn southeast(&self) -> Self {
        Self {
            x: self.x + T::one(),
            y: self.y + T::one(),
        }
    }

    #[inline]
    pub fn move_southwest(&mut self) -> &mut Self {
        self.move_down().move_left();

        self
    }

    #[inline]
    pub fn southwest(&self) -> Self {
        Self {
            x: self.x - T::one(),
            y: self.y + T::one(),
        }
    }

    pub fn move_next(&mut self, dir: Direction) -> &mut Self {
        use Direction::*;

        match dir {
            Up => self.move_up(),
            Right => self.move_right(),
            Down => self.move_down(),
            Left => self.move_left(),
        };

        self
    }

    pub fn next(&self, dir: Direction) -> Self {
        use Direction::*;

        match dir {
            Up => self.up(),
            Right => self.right(),
            Down => self.down(),
            Left => self.left(),
        }
    }

    pub fn move_previous(&mut self, dir: Direction) -> &mut Self {
        use Direction::*;

        match dir {
            Up => self.move_down(),
            Right => self.move_left(),
            Down => self.move_up(),
            Left => self.move_right(),
        };

        self
    }

    pub fn previous(&self, dir: Direction) -> Self {
        use Direction::*;

        match dir {
            Up => self.down(),
            Right => self.left(),
            Down => self.up(),
            Left => self.right(),
        }
    }

    pub fn neighbors(&self) -> [Coordinates<T>; 4] {
        [self.up(), self.right(), self.down(), self.left()]
    }

    pub fn neighbors_8(&self) -> [Coordinates<T>; 8] {
        [
            self.up(),
            self.northeast(),
            self.right(),
            self.southeast(),
            self.down(),
            self.southwest(),
            self.left(),
            self.northwest(),
        ]
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

#[derive(Debug, Error)]
pub enum TryFromStrError<E> {
    #[error("not enough elements")]
    NotEnoughElements,

    #[error("couldn't convert element into target type")]
    Parse(#[from] E),
}

impl<T> TryFrom<&str> for Coordinates<T>
where
    T: std::str::FromStr,
{
    type Error = TryFromStrError<<T as std::str::FromStr>::Err>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(',');

        let x = parts.next().ok_or(TryFromStrError::NotEnoughElements)?;
        let y = parts.next().ok_or(TryFromStrError::NotEnoughElements)?;

        let x = T::from_str(x)?;
        let y = T::from_str(y)?;

        Ok(Self::new(x, y))
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

impl<T: Add<Output = T>> Add for Coordinates<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Coordinates<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
