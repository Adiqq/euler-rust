#![allow(dead_code)]
mod common;
use std::panic;

pub fn problem3() {
    let mut x = 2;
    loop {
        let result = panic::catch_unwind(|| _problem3(x, 600851475143));
        if result.is_ok() {
            break;
        }
        x += 1;
    }
}

fn _problem3(x: i64, number: i64) {
    let (mut x_fixed, mut cycle_size, mut x, mut factor, mut count) = (x, 2, x, 1, 1);

    while factor == 1 {
        while count <= cycle_size && factor <= 1 {
            x = (x * x + 1) % number;
            factor = common::gcd(x - x_fixed, number);
            count += 1;
        }
        cycle_size *= 2;
        x_fixed = x;
    }

    println!("{}", factor);
}

//https://projecteuler.net/problem=2
pub fn problem2() {
    let mut current = 2;
    let mut prev = 1;
    let mut sum = 0;
    while current < 4 * 1000 * 1000 {
        if current % 2 == 0 {
            sum += current;
        }
        common::swap(&mut prev, &mut current);
        current += prev;
    }
    println!("{}", sum)
}
