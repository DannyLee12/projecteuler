/*
A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.
A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.
As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written
as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than
28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.
Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
*/
use std::time::Instant;

fn abundant(number: i32) -> bool {
    let mut i = 1;
    let mut total = 0;
    while i*2 <= number {
        if number % i == 0 {
            total += i
        }
        i += 1;
    }
    total > number
}

pub fn main() {
    let now = Instant::now();
    let mut total: i128 = 0;
    let mut v: Vec<i32> = vec![];
    assert!(abundant(12));
    // Create a list of abundant numbers
    for i in 0..28123 {
        if abundant(i) {
            v.push(i);
        }
    }
    for i in 0..28123 {
        let mut able = false;
        for j in v.iter() {
            if j > &i { break }
            if v.contains(&(i - j)) {
                able = true;
                break;
            }
        }
        if !able {
            total += i as i128;
        }
    }
    println!("Total is: {}", total);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}