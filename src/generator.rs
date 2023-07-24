use num::{bigint::ToBigInt, BigInt, ToPrimitive};
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

pub struct PiGenerator {
    curr: u32,
    next: u32,
}

impl PiGenerator {
    pub fn new() -> Self {
        Self { curr: 0, next: 1 }
    }

    pub fn start(offset: u32) -> Self {
        Self {
            curr: offset,
            next: (offset + 1),
        }
    }
}

impl Iterator for PiGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = self.curr + 1;

        Some(format!("{:x}", bbp(current).floor() as u32))
    }
}

// https://en.wikipedia.org/wiki/Bailey%E2%80%93Borwein%E2%80%93Plouffe_formula#BBP_digit-extraction_algorithm_for_%CF%80
fn series(n: u32, j: u32) -> f64 {
    let precision = 100;
    (0..n + 1)
        .into_par_iter()
        .map(|i| (modular_exponentiation(16, n - i, 8 * i + j) as f64 / (8 * i + j) as f64))
        .sum::<f64>()
        + (n + 1..precision)
            .into_par_iter()
            .map(|i| 16.0_f64.powi(-((i - n) as i32)) / ((8 * i + j) as f64))
            .sum::<f64>()
}

fn bbp(n: u32) -> f64 {
    let formula: [(i32, u32); 4] = [(4, 1), (-2, 4), (-1, 5), (-1, 6)];
    let res = formula
        .par_iter()
        .fold(
            || 0.0,
            |acc, (x, j)| {
                let series = series(n, *j);
                acc + (*x as f64 * series)
            },
        )
        .sum::<f64>();

    (res - res.floor()) * 16.0
}

// https://rosettacode.org/wiki/Modular_exponentiation#Rust
fn modular_exponentiation<T: ToBigInt>(n: T, e: T, m: T) -> u64 {
    let n = n.to_bigint().unwrap();
    let e = e.to_bigint().unwrap();
    let m = m.to_bigint().unwrap();

    assert!(e >= Zero::zero());

    use num::traits::{One, Zero};
    if e == Zero::zero() {
        return 1;
    }
    let mut result: BigInt = One::one();
    let mut base = n % &m;
    let mut exp = e;

    loop {
        if &exp % 2 == One::one() {
            result *= &base;
            result %= &m;
        }

        if exp == One::one() {
            return result.to_u64().unwrap();
        }

        exp /= 2;
        base *= base.clone();
        base %= &m;
    }
}

#[cfg(test)]
mod test {
    use crate::generator::bbp;

    #[test]
    fn bbp_test() {
        let first_digits_hex = "243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89";
        let mut string = String::new();
        for i in 0..first_digits_hex.len() {
            string += &(format!("{:x}", bbp(i as u32).floor() as u32));
        }
        assert_eq!(string.to_uppercase(), first_digits_hex)
    }
}
