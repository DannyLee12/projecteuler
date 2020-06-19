/*
The primes 3, 7, 109, and 673, are quite remarkable. By taking any two primes and concatenating them
in any order the result will always be prime. For example, taking 7 and 109, both 7109 and 1097 are
prime. The sum of these four primes, 792, represents the lowest sum for a set of four primes with
this property.

Find the lowest sum for a set of five primes for which any two primes concatenate to produce another
prime.
*/
use std::cmp::min;
use std::fs;
use std::time::Instant;


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
    let data = fs::read_to_string("/Users/daniellee/Development/projecteuler/projecteuler/src/primes_1mil.txt").expect("Unable to read");
    let primes: Vec<&str> = data.split(" ").collect();
    let mut total = u64::max_value();
    for i in 1..6 {
        for j in 0..1000 {
            if ! (is_prime(format!("{}{}", primes[i], primes[j]).parse::<u64>().unwrap()) && is_prime(format!("{}{}", primes[j], primes[i]).parse::<u64>().unwrap())) {
                continue;
            }
            for k in 0..1000 {
                if ! (is_prime(format!("{}{}", primes[i], primes[k]).parse::<u64>().unwrap()) && is_prime(format!("{}{}", primes[k], primes[i]).parse::<u64>().unwrap())
                    && is_prime(format!("{}{}", primes[j], primes[k]).parse::<u64>().unwrap()) && is_prime(format!("{}{}", primes[k], primes[j]).parse::<u64>().unwrap())) {
                    continue;
            }
                for l in 0..3000 {
                    if ! (is_prime(format!("{}{}", primes[i], primes[l]).parse::<u64>().unwrap()) && is_prime(format!("{}{}", primes[l], primes[i]).parse::<u64>().unwrap())
                        && is_prime(format!("{}{}", primes[j], primes[l]).parse::<u64>().unwrap()) && is_prime(format!("{}{}", primes[l], primes[j]).parse::<u64>().unwrap())
                        && is_prime(format!("{}{}", primes[k], primes[l]).parse::<u64>().unwrap()) && is_prime(format!("{}{}", primes[l], primes[k]).parse::<u64>().unwrap())){
                        continue;
                    }
                    for m in 0..3000 {
                        if is_prime(format!("{}{}", primes[i], primes[m]).parse::<u64>().unwrap()) && is_prime(format!("{}{}", primes[m], primes[i]).parse::<u64>().unwrap())
                            && is_prime(format!("{}{}", primes[j], primes[m]).parse::<u64>().unwrap()) && is_prime(format!("{}{}", primes[m], primes[j]).parse::<u64>().unwrap())
                            && is_prime(format!("{}{}", primes[k], primes[m]).parse::<u64>().unwrap()) && is_prime(format!("{}{}", primes[m], primes[k]).parse::<u64>().unwrap())
                            && is_prime(format!("{}{}", primes[l], primes[m]).parse::<u64>().unwrap()) && is_prime(format!("{}{}", primes[m], primes[l]).parse::<u64>().unwrap())
                        {
                            total = min(total, primes[i].parse::<u64>().unwrap() + primes[j].parse::<u64>().unwrap() + primes[k].parse::<u64>().unwrap() + primes[l].parse::<u64>().unwrap() + primes[m].parse::<u64>().unwrap());
                            println!("Total is: {}", total);
                        }
                    }
                }
            }

        }
    }
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}