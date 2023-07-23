use std::fs::File;

use generator::PiGenerator;
use image::{Rgb, RgbImage};
use memmap2::MmapOptions;

mod generator;

const DECIMAL_TO_BINARY: [u16; 10] = [0000, 0001, 0010, 0011, 0100, 0101, 0110, 0111, 1000, 1001];

fn main() {
    let x = 100;
    let y = 100;
    let pi_offset = 500000;

    // let num_chars = (x * y * 3 * 2) as usize;
    // let decimal_string = get_pi_from_file("pi_1m.txt".to_string(), pi_offset, num_chars);

    // println!("writing image");

    // let img = generate_image(x, y, decimal_string);
    // img.save("test.png").unwrap();

    let mut generator = PiGenerator::new();
    let mut hex_string = String::new();

    // time how long this takes

    let now = std::time::Instant::now();
    for _ in 0..(x * y) {
        hex_string.push_str(&generator.next().unwrap());
    }

    println!("{hex_string}\ntime: {:?}", now.elapsed());
}

fn get_pi_from_file(file_name: String, offset: usize, num_chars: usize) -> String {
    let file = File::open(file_name).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    String::from_utf8(mmap.get(offset..(offset + num_chars)).unwrap().to_vec()).unwrap()
}

fn generate_image(x: u32, y: u32, decimal_string: String) -> RgbImage {
    let mut img = RgbImage::new(x, y);

    let chunks = decimal_string_to_byte_slice(&decimal_string)
        .chunks(3)
        .map(|slice| {
            let mut src = [0; 3];
            src[..slice.len()].copy_from_slice(slice);
            Rgb(src)
        })
        .collect::<Vec<Rgb<u8>>>();

    (0..x)
        .flat_map(|i| (0..y).map(move |j| (i, j)))
        .zip(chunks.into_iter())
        .for_each(|((x, y), pixel)| img.put_pixel(x, y, pixel));

    img
}

fn decimal_string_to_byte_slice(string: &str) -> Vec<u8> {
    let mut bytes = vec![];

    let digits = string
        .chars()
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
