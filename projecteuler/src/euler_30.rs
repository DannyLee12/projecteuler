/*
Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:

    1634 = 14 + 64 + 34 + 44
    8208 = 84 + 24 + 04 + 84
    9474 = 94 + 44 + 74 + 44

As 1 = 14 is not a sum it is not included.

The sum of these numbers is 1634 + 8208 + 9474 = 19316.

Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
*/

use std::time::Instant;


pub fn main() {
    let now = Instant::now();
    let power = 5;
    let mut total = 0;
    let mut count = 0;
    for i in 2..10000000_u128 {
        let s = i.to_string();
        let mut sum = 0;
        for c in s.chars() {
            sum += u128::pow(c as u128 - '0' as u128, power);
//            println!("c: {}, c as i32: {}, sum: {}",c, c as i32, sum);
        }
        if sum == i {
            println!("{}", sum);
            total += sum;
            count += 1;
        }
    }
    println!("Total: {}", total);
    println!("Count: {}", count);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}