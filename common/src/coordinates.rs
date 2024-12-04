use num_traits::{Float, NumCast, PrimInt, ToPrimitive};
use std::ops::{Add, AddAssign, SubAssign};

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

impl<T: Add<Output = T>> Add for Coordinates<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
