/*
The number 3797 has an interesting property. Being prime itself, it is possible to continuously
remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we
can work from right to left: 3797, 379, 37, and 3.

Find the sum of the only eleven primes that are both truncatable from left to right and right to
left.
*/

use std::time::Instant;
use std::fs;


pub fn main() {
    let now = Instant::now();
    let data = fs::read_to_string("/Users/daniellee/Development/projecteuler/projecteuler/src/primes.txt").expect("Unable to read file");
    let primes: Vec<&str> = data.split(" ").collect();
    let mut total: u64 = 0;
    for (i, prime) in data.split(" ").enumerate() {
        if i < 4 { continue };
        let l = prime.len() - 1;
        let mut truncatable = true;
        for j in 0..l {
            let s1: String = prime.chars().skip(0).take(j+1).collect();
            let s2: String = prime.chars().skip(j+1).take(l-j).collect();
            if !(primes.contains(&&s1[..]) && primes.contains(&&s2[..])) {
                truncatable = false;
                break;
            }
        }
        if truncatable {
            println!("{}", prime);
            total += prime.parse::<u64>().unwrap();
        }
    }
    println!("Total is: {}", total);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}
