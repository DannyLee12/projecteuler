/*
There are exactly ten ways of selecting three from five, 12345:

123, 124, 125, 134, 135, 145, 234, 235, 245, and 345

In combinatorics, we use the notation, (5C3)=10

In general, (nr)=n!/r!(n−r)!
, where r≤n, n!=n×(n−1)×...×3×2×1, and 0!=1

It is not until n=23
, that a value exceeds one-million: (23C10)=1144066

How many, not necessarily distinct, values of (nr)
for 1 ≤ n ≤ 100, are greater than one-million?
*/

extern crate num_bigint;
use std::time::Instant;
use std::collections::HashMap;

use num_bigint::{BigUint, ToBigUint};

fn factorial(n: BigUint) -> BigUint {
//    println!("{}", n);
    if n == ToBigUint::to_biguint(&1).unwrap() {
        return ToBigUint::to_biguint(&1).unwrap()
    }
    let value = factorial(&n - ToBigUint::to_biguint(&1).unwrap());

    n * value
}

//fn xCy (x: BigUint, y: BigUint) -> u128 {
//    let mut val = 1;
//    let mut counter = 1;
//    println!("{}, {}", x, y);
//    if y > x - y {
//        for i in 0..(u128::from(x) - u128::from(y)) {
//            val *= x - i.to_bigint().unwrap();
//        }
//        val /= factorial(&(x - y));
//    } else {
//        for j in 0..y {
//            val *= x - j;
//        }
//        val /= factorial(y);
//    }
//}

pub fn main() {
    let now = Instant::now();
    let mut total = 0;
    for n in 23..101 {
        for r in 1..n {
            let bn = ToBigUint::to_biguint(&n).unwrap();
            let br = ToBigUint::to_biguint(&r).unwrap();
            let bd = ToBigUint::to_biguint(&(n-r)).unwrap();
            let val: BigUint = factorial(bn)/(factorial(br) * factorial(bd));
            if val > ToBigUint::to_biguint(&1000000).unwrap() {
                total += 1;
//                println!("{}", val);
            }
        }
    }
    println!("Total: {}", total);
//    println!("{}", u128::max_value());
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}