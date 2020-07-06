/*
The 5-digit number, 16807=75, is also a fifth power. Similarly, the 9-digit number, 134217728=89,
is a ninth power.

How many n-digit positive integers exist which are also an nth power?
*/
use std::time::Instant;

pub fn main() {
    let now = Instant::now();
    let mut total = 0;
    for i in 1..30 {
        let mut j: u128 = 1;
        loop {
            if j >= 10 {
                break
            }
            if j.pow(i) < 10_u128.pow(i - 1) {
                j += 1;
            }
            else if j.pow(i) > 10_u128.pow(i) {
                break;
            } else {
                println!("{}", j.pow(i));
                total += 1;
                j += 1;
            }
        }
    }
    println!("Total numbers: {}", total);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}
