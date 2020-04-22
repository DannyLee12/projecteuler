/*
Take the number 192 and multiply it by each of 1, 2, and 3:

    192 × 1 = 192
    192 × 2 = 384
    192 × 3 = 576

By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576 the
concatenated product of 192 and (1,2,3)
The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the
pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).
What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product
of an integer with (1,2, ... , n) where n > 1?
*/

use std::time::Instant;
use std::cmp;

pub fn main() {
    let now = Instant::now();
    let mut n: u128 = 1;
    let mut num: u128 = 191;
    let mut largest = 0;
    let mut val = "".to_string();
    let mut total: u128 = 0;
    loop {
        val = "".to_string();
        total = 0;
        n = 1;
        while total <= 987654321 {
            val.push_str(&((n * num).to_string()));
            total = val.parse().unwrap();
            if total >= 123456789 && total <= 987654321 {
                let mut pangental = true;
                for j in 1..10 {
                    if ! val.contains(&(j.to_string())) {
                        pangental = false;
                        break;
                    }
                }
                if pangental {
                    largest = cmp::max(largest, total);
                    println!("{}, {}", largest, total);
                }
            }
            n += 1;
        }
        num += 1;
        if num > 987654321 { break };
    }
    println!("{}", largest);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}