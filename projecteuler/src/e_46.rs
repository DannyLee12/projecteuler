/*
The first two consecutive numbers to have two distinct prime factors are:

14 = 2 × 7
15 = 3 × 5

The first three consecutive numbers to have three distinct prime factors are:

644 = 2² × 7 × 23
645 = 3 × 5 × 43
646 = 2 × 17 × 19.

Find the first four consecutive integers to have four distinct prime factors each. What is the first
of these numbers?
*/

use std::time::Instant;
use std::fs;


pub fn main() {
    let now = Instant::now();
    let data = fs::read_to_string("/Users/daniellee/Development/projecteuler/projecteuler/src/primes.txt").expect("Unable to read file");
    let primes: Vec<&str> = data.split(" ").collect();
    let mut v: Vec<u64> = vec![0; 2000000];
    for prime in primes {
        let mut counter = 1;
        loop {
            let val = prime.parse::<u64>().unwrap() * counter;
            if val >= 2000000 { break }
            v[val as usize] += 1;
            counter += 1;
        }
    }
    println!("{:?}", v);
    for i in 1..199996 {
        if v[i] == 4 && v[i+1] == 4 && v[i+2] == 4 && v[i+3] == 4 {
            println!("{}, {:?}", i, &v[i..i+4]);
        }
    };
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}