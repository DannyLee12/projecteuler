/*
Starting with 1 and spiralling anticlockwise in the following way, a square spiral with side length
7 is formed.

37 36 35 34 33 32 31
38 17 16 15 14 13 30
39 18  5  4  3 12 29
40 19  6  1  2 11 28
41 20  7  8  9 10 27
42 21 22 23 24 25 26
43 44 45 46 47 48 49

It is interesting to note that the odd squares lie along the bottom right diagonal, but what is more
interesting is that 8 out of the 13 numbers lying along both diagonals are prime; that is, a ratio
of 8/13 ≈ 62%.

If one complete new layer is wrapped around the spiral above, a square spiral with side length 9
will be formed. If this process is continued, what is the side length of the square spiral for
which the ratio of primes along both diagonals first falls below 10%?
*/

use std::time::Instant;
use itertools::Itertools;

pub fn is_prime(n: u64) -> bool {
    if n <= 1 { return false } ;
    if n == 2 {return true};
    if n % 2 == 0 { return false};
    if n < 9 { return true};
    if n % 3 == 0 { return false}

    let mut counter = 5;
    while (counter * counter) <= n {
        if n % counter == 0 {
          return false;
        }
        if n % (counter + 2) == 0 {
            return false
        }
        counter += 6;
    }
    true
}

pub fn main() {
    let now = Instant::now();
    let mut d_tr = 3; // Top right
//    let mut d_br = 9; //Ignore
    let mut d_bl = 7;
    let mut d_tl = 5;
    let mut total_primes: f64 = 3.0;
    let mut total_numbers: f64 = 5.0;
    let mut ratio: f64 = 0.75; // Ratio in Percentage to avoid floating points
    let mut layer = 0;
    while ratio >= 0.1 {
        d_tr += 10 + layer * 8;
        d_tl += 12 + layer * 8;
        d_bl += 14 + layer * 8;
        if is_prime(d_tr) {
            total_primes += 1.0
        }
        if is_prime(d_tl){
            total_primes += 1.0
        }
        if is_prime(d_bl) {
            total_primes += 1.0
        }
        total_numbers += 4.0;
        layer += 1;
        ratio = total_primes / total_numbers;
    }
    println!("Ratio: {}, Layer: {}", ratio, layer);
    println!("Side Length: {}", 2 * (layer + 1) + 1);
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}
