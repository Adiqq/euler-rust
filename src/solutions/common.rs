#![allow(dead_code)]
#![allow(unused_assignments)]
pub fn swap(x: &mut i32, y: &mut i32) {
    if x != y {
        *x ^= *y;
        *y ^= *x;
        *x ^= *y;
    }
}
pub fn gcd(x: i64, y: i64) -> i64 {
    let mut a = x;
    let mut b = y;
    let mut remainder = 0;
    while b != 0 {
        remainder = a % b;
        a = b;
        b = remainder;
    }
    return a;
}
