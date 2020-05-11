/*
The prime 41, can be written as the sum of six consecutive primes:
41 = 2 + 3 + 5 + 7 + 11 + 13

This is the longest sum of consecutive primes that adds to a prime below one-hundred.

The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms, and is equal to 953.

Which prime, below one-million, can be written as the sum of the most consecutive primes?
*/

use std::time::Instant;
use std::fs;

pub fn main() {
    let now = Instant::now();
    let data = fs::read_to_string("/Users/daniellee/Development/projecteuler/projecteuler/src/primes_1mil.txt").expect("Unable to read file");
    let primes2: Vec<&str> = data.split(" ").collect();
    let mut max_value = 0;
    let mut max_length = 0;
    let mut counter = 0;
    let mut start = 0;
    let mut total: u64 = 0;
    let mut number= 0;
    while start < 10 {
        let primes: Vec<&str> = data.split(" ").collect();
        for prime in primes {
            counter += 1;
            if counter < start {continue};
            let p = prime.parse::<u64>().unwrap();
            total += p;
            number += 1;
            if total > 1000000 { break } ;
            if primes2.contains(&(&total.to_string()[..])) {
                if number > max_length {
                    max_value = total;
                    max_length = number;
                }
             }
        }
        counter = 0;
        start += 1;
        total = 0;
        number = 0;
    }

    println!("{}, {}", max_length, max_value);
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}