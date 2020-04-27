/*


An irrational decimal fraction is created by concatenating the positive integers:

0.123456789101112131415161718192021...

It can be seen that the 12th digit of the fractional part is 1.

If dn represents the nth digit of the fractional part, find the value of the following expression.

d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000

*/

use std::time::Instant;


pub fn main() {
    let now = Instant::now();
    let mut s = "".to_string();

    for i in 0..1000000u128 {
        s.push_str(&i.to_string())
    }
    let mut total: u128 = 1;
    let mut j = 1;
    while j <= 1000000 {
        total *= s.chars().nth(j).unwrap() as u128 - '0' as u128;
        j *= 10;
    }
    println!("{}", total);
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}