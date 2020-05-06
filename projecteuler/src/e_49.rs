/*
The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is unusual
in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers are
permutations of one another.

There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this
property, but there is one other 4-digit increasing sequence.

What 12-digit number do you form by concatenating the three terms in this sequence?

*/

use std::fs;
use std::time::Instant;
use itertools::Itertools;

pub fn main() {
    let now = Instant::now();
    let data = fs::read_to_string("/Users/daniellee/Development/projecteuler/projecteuler/src/primes.txt").expect("Unable to read file");
    let primes: Vec<&str> = data.split(" ").collect();
    let primes2: Vec<&str> = data.split(" ").collect();

    for prime in primes {
        if prime.len() < 4 {
            continue
        }
        else if prime.len() > 4 {
            break
        }
        let p: Vec<char> = prime.chars().collect();
        let s: String = p.into_iter().collect();
        let p2: i32 = s.parse::<i32>().unwrap() + 3330;
        let p3: i32 = p2 + 3330;
        let p2s = &p2.to_string()[..];
        let p3s = &p3.to_string()[..];
        if p3 > 9999 {break}
        if primes2.contains(&p2s) && primes2.contains(&p3s) {
            let mut valid = true;
            for c in s.chars() {
                if !(p2s.contains(c) && p3s.contains(c)) {
                    valid = false;
                    break;
                }
            }
            if valid {
                println!("{}{}{}", s, p2s, p3s)
            }
        }
    }
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}