/*
It was proposed by Christian Goldbach that every odd composite number can be written as the sum of
a prime and twice a square.

9 = 7 + 2×12
15 = 7 + 2×22
21 = 3 + 2×32
25 = 7 + 2×32
27 = 19 + 2×22
33 = 31 + 2×12

It turns out that the conjecture was false.

What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?

*/

use std::time::Instant;
use std::fs;


pub fn main() {
    let now = Instant::now();
    let data = fs::read_to_string("/Users/daniellee/Development/projecteuler/projecteuler/src/primes2.txt").expect("Unable to read file");
    let list: Vec<&str> = data.split(" ").collect();
    let list2: Vec<&str> = data.split(" ").collect();
    let mut v: Vec<u64> = vec![];

    for prime in list {
        for i in 1..100 {
            let val  = prime.parse::<u64>().unwrap() + 2 * (i*i);
            if val % 2 != 0 && !v.contains(&val) {
                v.push(val);
            }
        }
    }
    for i in 2..150000 {
        let s = i.to_string();
        let ss: &str = &s[..];
        if i % 2 == 0 {
            continue
        }
        else if list2.contains(&ss) {
            continue
        }
        else if v.contains(&i) {
            continue
        }
        println!("{}", i);
        break;
    }
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}