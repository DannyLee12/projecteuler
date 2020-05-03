/*
We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.

The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.

Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.
HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.
*/
use std::time::Instant;


pub fn main() {
    let now = Instant::now();
    let mut v: Vec<u128> = vec![];
    for x in 0..10000u128 {
        for y in x..10000u128 {
            let mut valid = true;
            let total_val = x * y;
            let mut total = total_val.to_string();
            total.push_str(&(x.to_string()));
            total.push_str(&(y.to_string()));
            if total.bytes().count() != 9 { continue }
            for i in 1..10 {
                if !total.contains(&(i.to_string())){
                    valid = false;
                    break
                }
            }
         if valid {
             if !v.contains(&total_val){
                 v.push(total_val);
                 println!("{:?}", v);
             }
         }
        }
    }
    let mut total = 0;
    for i in v {
        total += i;
    }
    println!("total is: {}", total);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}