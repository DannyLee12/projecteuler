/*
By replacing the 1st digit of the 2-digit number *3, it turns out that six of the nine possible
values: 13, 23, 43, 53, 73, and 83, are all prime.

By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number is the first
example having seven primes among the ten generated numbers, yielding the family: 56003, 56113,
56333, 56443, 56663, 56773, and 56993. Consequently 56003, being the first member of this family,
is the smallest prime with this property.

Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits)
with the same digit, is part of an eight prime value family.
*/

use std::time::Instant;
use std::fs;

fn get_pots() {
    let data = fs::read_to_string("/Users/daniellee/Development/projecteuler/projecteuler/src/primes_1mil.txt").expect("Unable to read file");
    let primes: Vec<&str> = data.split(" ").collect();
    for i in 120000..999999 {
        let is = i.to_string();
        if primes.contains(&(&is[..])){
            if is.matches('1').count() == 3  && is.chars().nth(5).unwrap() == '3'{
                println!("{}", i);
            }
        }
    }
}

pub fn main() {
    let now = Instant::now();
    let data = fs::read_to_string("/Users/daniellee/Development/projecteuler/projecteuler/src/primes_1mil.txt").expect("Unable to read file");
    let primes: Vec<&str> = data.split(" ").collect();

    let v = vec![121013, 121113, 121313];

    for val in v {
        let mut total = 0;
        let s = val.to_string();
        for i in 1..10 {
            let sr = s.replace('0', &i.to_string());
            if primes.contains(&(&sr[..])) {
                total += 1;
            }
            if total == 8 {
                println!("{}", val);
                break
            }
        }
    }
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}