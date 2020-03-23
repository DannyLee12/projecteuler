/*

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

pub fn sieve_erat(range: u64) -> Vec<u64> {
    let mut v = vec![];
    for i in 2..range {
        if i * i > range { break };
        for j in (i*i..range).step_by(i as usize) {
            if !v.contains( & j){
                v.push( j)
            }
        }
    }
    v
}

pub fn main() {
    let sieve = sieve_erat(150010);
    let mut count = 0;
    for i in 2..300000000u64 {
        if !sieve.contains(&i) {
            println!("{}", i);
            count += 1;
        }
        if count == 10001 {
            println!("{}", i);
            break;
        }
    }

}

/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.

*/

use std::fs;

pub fn main_10() {
    let mut total: u64 = 0;
    let data = fs::read_to_string("src/primes.txt").expect("Unable to read file");
    let list = data.split(" ").collect::<Vec<&str>>();
    println!("{}", data.len());
    for i in list {
//        println!("{}", i);
        total += i.parse::<u64>().unwrap() as u64;
    }
    println!("{}", total);
}
