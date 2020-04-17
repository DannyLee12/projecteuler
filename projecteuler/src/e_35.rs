/*
The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.

There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

How many circular primes are there below one million?
*/

use std::time::Instant;
use std::fs;
use itertools::Itertools;

pub fn main() {
    let now = Instant::now();
    let data = fs::read_to_string("/Users/daniellee/Development/projecteuler/projecteuler/src/primes.txt").expect("Unable to read file");
    let primes: Vec<&str> = data.split(" ").collect();
    let mut total = 0;
    let mut counter = 0;
    for prime in primes {
        if counter % 5000 == 0 {
            println!("{}", counter);
            println!("{}", prime);
        }
        counter += 1;
        if prime.parse::<u64>().unwrap() > 2000001 { break };
        if prime.bytes().count() == 1 {
            total += 1;
        }
        else {
            let mut circular = true;
            let v: Vec<char> = prime.chars().collect();
            for p in v.iter().permutations(v.len()).unique() {
                let mut s: String = p.into_iter().collect();
                s.push_str(" ");
                let s = format!(" {}", s);
//                println!("{}", s);
                if ! data.contains(&s) {
                    circular = false;
                    break;
                }
            }
            if circular {
                total += 1;
//                println!("Total: {}, Prime: {}", total, prime);
                if total % 50 == 0 {
                    println!("Total: {}", total);
                }
            }
        }
    }
    println!("{}", total);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}