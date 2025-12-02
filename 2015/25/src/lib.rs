use common::Answer;

type IntType = u64;

fn pow(mut base: IntType, mut exp: IntType, modulus: IntType) -> IntType {
    let mut result = 1;

    base %= modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }

        base = (base * base) % modulus;
        exp /= 2;
    }

    result
}

fn nth(row: IntType, col: IntType) -> IntType {
    (row + col - 2) * (row + col - 1) / 2 + col
}

fn nth_code(n: IntType) -> IntType {
    (20151125 * pow(252533, n - 1, 33554393)) % 33554393
}

pub fn step1(_: &str) -> Answer {
    nth_code(nth(3010, 3019)).into()
}

pub fn step2(_: &str) -> Answer {
    ().into()
}
