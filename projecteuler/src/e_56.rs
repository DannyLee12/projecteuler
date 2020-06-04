/*
A googol (10100) is a massive number: one followed by one-hundred zeros; 100100 is almost
unimaginably large: one followed by two-hundred zeros. Despite their size, the sum of the digits in
each number is only 1.

Considering natural numbers of the form, ab, where a, b < 100, what is the maximum digital sum?

*/

use std::time::Instant;
use num_bigint::{BigUint, ToBigUint, BigInt};
extern crate num;
use std::cmp::max;

pub fn main() {
    let now = Instant::now();
    let max_val: BigUint = ToBigUint::to_biguint(&0).unwrap();
    let mut max_val = 0;
    for a in 1..101 {
        for b in 1..101 {
            let mut sum = 0;
            let ba = ToBigUint::to_biguint(&a).unwrap();
            let val = num::pow(ba, b);
            let s: String = format!("{}", val);
            for c in s.chars() {
                sum += c as u128 - '0' as u128
            }
            max_val = max(max_val, sum);
        }
    }
    println!("Max decimal sum: {}", max_val);
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}
