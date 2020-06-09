/*
It is possible to show that the square root of two can be expressed as an infinite continued
fraction.

2–√=1+12+12+12+…

By expanding this for the first four iterations, we get:

1+12=32=1.5

1+12+12=75=1.4
1+12+12+12=1712=1.41666…
1+12+12+12+12=4129=1.41379…


The next three expansions are 9970
, 239169, and 577408, but the eighth expansion, 1393985

, is the first example where the number of digits in the numerator exceeds the number of digits in
the denominator.

In the first one-thousand expansions, how many fractions contain a numerator with more digits than
the denominator?
*/

use std::time::Instant;


pub fn main() {
    let now = Instant::now();
    let mut num: u128 = 5;
    let mut dem = 2;
    let mut total = 0;
    for i in 1..1001 {
        // Remove very large numbers
        if num > 1000000000 {
            num /= 10000;
            dem /= 10000;
        }
        // 2 + num/dem
        if i > 1 {
            num += 2 * dem;
        }
        // (1 / num/dem) = dem/num
        let temp = num;
        num = dem;
        dem = temp;
        // 1 + num / dem
        if (num+dem).to_string().len() > dem.to_string().len() {
            total += 1;
        }
    }
    println!("Total number with num > dem = {}", total);
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}
