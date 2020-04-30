/*


The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.
*/
use std::time::Instant;

fn collatz(value: u64, mut length: i32) -> i32 {
    length += 1;
    if value == 1 { return length }
    else if value % 2 == 0 { collatz(value/2, length) }
    else { collatz((3 * value) + 1, length) }
}

pub fn main() {
    let now = Instant::now();
    let mut max: i32 = 0;
    let mut val: u64 = 0;
    for i in 2..1000000 {
        let length = collatz(i, 0);
        if length > max {
            max = length;
            val = i;
        }
        println!("{}, {}", max, val);
    }
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}
