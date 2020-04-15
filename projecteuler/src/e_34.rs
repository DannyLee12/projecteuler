/*
145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.

Find the sum of all numbers which are equal to the sum of the factorial of their digits.

Note: as 1! = 1 and 2! = 2 are not sums they are not included.
*/

use std::time::Instant;

pub fn main() {
    let now = Instant::now();
    let mut total_of_totals: u128 = 0;
    for i in 3..40586 {
        let mut total = 0;
        let mut n = i;
        while n > 0 {
            total += factorial(n % 10);
            n /= 10;
        }
        if total == i {
            println!("{}", i);
            total_of_totals += i;
        }
    }
    assert_eq!(factorial(8), 40320);
    println!("{}", total_of_totals);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}

fn factorial(val: u128) -> u128 {
    if val <= 1 {
        return 1;
    }
    return val * factorial(val - 1);
}