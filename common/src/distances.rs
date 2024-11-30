use crate::Coordinates;
use std::ops::{Add, Sub};

pub fn manhattan<T>(a: Coordinates<T>, b: Coordinates<T>) -> T
where
    T: Copy + PartialOrd + Sub<Output = T> + Add<Output = T>,
{
    let x = if a.x() > b.x() {
        a.x() - b.x()
    } else {
        b.x() - a.x()
    };

    let y = if a.y() > b.y() {
        a.y() - b.y()
    } else {
        b.y() - a.y()
    };

    x + y
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_manhattan_unsigned() {
        let a = Coordinates::new(18u32, 55u32);
        let b = Coordinates::new(0u32, 0u32);

        assert_eq!(manhattan(a, b), 73);
    }

    #[test]
    fn test_manhattan_signed() {
        let a = Coordinates::new(2i16, 9);
        let b = Coordinates::new(3, 5);

        assert_eq!(manhattan(a, b), 5);
    }

    #[test]
    fn test_manhattan_extreme() {
        let a = Coordinates::new(1292120301, 1992919919);
        let b = Coordinates::new(-19228391, 293849111);

        assert_eq!(manhattan(a, b), 3010419500i64);
    }
}
