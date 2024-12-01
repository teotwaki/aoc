use crate::Coordinates;
use num_traits::{Float, Num};

pub fn manhattan<T>(a: Coordinates<T>, b: Coordinates<T>) -> T
where
    T: Copy + Num + PartialOrd,
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

pub fn euclidian<T>(a: Coordinates<T>, b: Coordinates<T>) -> T
where
    T: Float,
{
    ((a.x() - b.x()).powi(2) + (a.y() - b.y()).powi(2)).sqrt()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn manhattan_works_with_unsigned() {
        let a = Coordinates::new(18u32, 55u32);
        let b = Coordinates::new(0u32, 0u32);

        assert_eq!(manhattan(a, b), 73);
    }

    #[test]
    fn manhattan_works_with_signed() {
        let a = Coordinates::new(2i16, 9);
        let b = Coordinates::new(3, 5);

        assert_eq!(manhattan(a, b), 5);
    }

    #[test]
    fn manhattan_works_with_floats() {
        let a = Coordinates::new(2.0, 9.0);
        let b = Coordinates::new(3.0, 5.0);

        assert_eq!(manhattan(a, b), 5.0);
    }

    #[test]
    fn manhattan_works_with_extreme() {
        let a = Coordinates::new(1292120301, 1992919919);
        let b = Coordinates::new(-19228391, 293849111);

        assert_eq!(manhattan(a, b), 3010419500i64);
    }

    #[test]
    fn euclidian_works_with_f32() {
        let a = Coordinates::new(3.0, 5.0);
        let b = Coordinates::new(9.0, 2.0);

        assert_eq!(euclidian(a, b), 45.0f32.sqrt());
    }

    #[test]
    fn euclidian_works_with_f64() {
        let a = Coordinates::new(13.0, 25.0);
        let b = Coordinates::new(9.0, -92.0);

        assert_eq!(euclidian(a, b), 13705.0f64.sqrt());
    }
}
