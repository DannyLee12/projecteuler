/*
The square root of 2 can be written as an infinite continued fraction.

The infinite continued fraction can be written, , indicates that 2 repeats ad infinitum. In a similar way,

.

It turns out that the sequence of partial values of continued fractions for square roots provide the best rational approximations. Let us consider the convergents for

.

Hence the sequence of the first ten convergents for

are:

What is most surprising is that the important mathematical constant,

.

The first ten terms in the sequence of convergents for e are:

The sum of digits in the numerator of the 10th convergent is

.

Find the sum of digits in the numerator of the 100th convergent of the continued fraction for
.
*/

use std::time::Instant;
use num_bigint::ToBigUint;
use num::BigUint;

pub fn main() {
    let now = Instant::now();
    let mut counter = 2;
    let mut i_2 = ToBigUint::to_biguint(&2).unwrap();
    let mut i_1 = ToBigUint::to_biguint(&3).unwrap();
    let mut p_1: BigUint;
    let mut k = ToBigUint::to_biguint(&1).unwrap();
        loop {
            if counter % 3 == 2 {
                p_1 = &k * &ToBigUint::to_biguint(&2).unwrap();
                k += ToBigUint::to_biguint(&1).unwrap();
            } else {
                p_1 = ToBigUint::to_biguint(&1).unwrap();
            }
            if counter == 99 {break};
            let num = &i_2 + &i_1 * &p_1;
            i_2 = i_1;
            i_1 = num;
            counter += 1;
        }
        let val = format!("{}", &i_2 + &i_1 * &p_1);
        let mut total = 0;
        for s in val.chars() {
            total += s as u64 - '0' as u64;
        }
        println!("Total is {}", total);
        println!("Script took {} milliseconds to run", now.elapsed().as_millis());

}