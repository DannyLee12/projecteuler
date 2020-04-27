/*
We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly
once. For example, 2143 is a 4-digit pandigital and is also prime.

What is the largest n-digit pandigital prime that exists?
*/

use std::fs;
use std::time::Instant;

pub fn main() {
    let now = Instant::now();
    let data = fs::read_to_string("/Users/daniellee/Development/projecteuler/projecteuler/src/primes1.txt").expect("Unable to read file");
    let list: Vec<&str> = data.split(" ").collect();
    let mut largest = "";
    for prime in list {
        let l = prime.chars().count();
        if l > 10 { break };
        let mut pandigital = true;
        for x in 1..l + 1 {
            if ! prime.contains(&x.to_string()) {
                pandigital = false;
                break;
            }
        }
        if pandigital {
            largest = prime;
        }
    }
    println!("{}", largest);
    println!("Script took {} seconds to run", now.elapsed().as_millis());
}