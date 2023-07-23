use std::{collections::HashMap, fs::File};

use image::{Rgb, RgbImage};
use memmap2::MmapOptions;
use num::{bigint::ToBigInt, BigInt, ToPrimitive};

const DECIMAL_TO_BINARY: [u16; 10] = [0000, 0001, 0010, 0011, 0100, 0101, 0110, 0111, 1000, 1001];

fn main() {
    let x = 256;
    let y = 256;
    let pi_offset = 500000;

    let mut img = RgbImage::new(x, y);

    let file = File::open("pi_1m.txt").unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };

    let num_chars = (x * y * 3 * 2) as usize;
    let string = String::from_utf8(
        mmap.get(pi_offset..(pi_offset + num_chars))
            .unwrap()
            .to_vec(),
    )
    .unwrap();

    let chunks = decimal_string_to_byte_slice(&string)
        .chunks(3)
        .map(|slice| {
            let mut src = [0; 3];
            src[..slice.len()].copy_from_slice(slice);
            Rgb(src)
        })
        .collect::<Vec<Rgb<u8>>>();

    (0..x)
        .into_iter()
        .flat_map(|i| (0..y).into_iter().map(move |j| (i, j)))
        .zip(chunks.into_iter())
        .for_each(|((x, y), pixel)| img.put_pixel(x, y, pixel));

    println!("writing image");
    img.save("test.png").unwrap();
}

fn decimal_string_to_byte_slice(string: &str) -> Vec<u8> {
    let mut bytes = vec![];

    let digits = string
        .chars()
        .into_iter()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<_>>>()
        .expect("not a number");

    let mut count = 0;
    let mut cur_byte = 0;
    for i in digits {
        cur_byte = (cur_byte << 4) | DECIMAL_TO_BINARY[i as usize];
        count += 1;
        if count == 2 {
            bytes.push(cur_byte as u8);
            count = 0;
            cur_byte = 0;
        }
    }

    bytes
}

// https://en.wikipedia.org/wiki/Bailey%E2%80%93Borwein%E2%80%93Plouffe_formula#BBP_digit-extraction_algorithm_for_%CF%80
fn series(n: u32, j: u32) -> f64 {
    let precision = 100;
    (0..n + 1)
        .into_iter()
        .map(|i| (modular_exponentiation(16, n - i, 8 * i + j) as f64 / (8 * i + j) as f64))
        .sum::<f64>()
        + (n + 1..precision)
            .into_iter()
            .map(|i| 16.0_f64.powi(-((i - n) as i32)) / ((8 * i + j) as f64))
            .sum::<f64>()
}

fn bbp(n: u32) -> f64 {
    let formula: [(i32, u32); 4] = [(4, 1), (-2, 4), (-1, 5), (-1, 6)];
    let init: f64 = 0.0;
    let res = formula.iter().fold(init, |acc, (x, j)| {
        let series = series(n, *j);
        acc + (*x as f64 * series)
    });

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

mod test {
    use crate::bbp;

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
