/*
Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.
*/

use std::time::Instant;
use std::collections::HashMap;

fn get_divisors(value: i32) -> i32 {
    let mut v: i32 = 0;
    let mut i = 1;
    while i*2 <= value {
        if value % i == 0 {
            v += i;
        }
        i += 1;
    }
    v
}

pub fn main() {
    println!("{}", get_divisors(6));
    let now = Instant::now();
    let mut h = HashMap::new();
    let mut total: i128 = 0;
    for i in 1..10001 {
        h.insert(i, get_divisors(i));
    }

    for i in 3..10001 {
        let mut spd: i32;
        let mut spd2: i32;
        spd = h[&i];
        if spd >= 10000 {continue};
        spd2 = h[&spd];
        if spd2 == spd {continue};
        if i == spd2 {
            println!{"{}", i};
            total += i as i128;
        }

    }
    println!("{}", total);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}
