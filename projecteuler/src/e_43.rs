/*
Pentagonal numbers are generated by the formula, Pn=n(3n−1)/2. The first ten pentagonal numbers are:

1, 5, 12, 22, 35, 51, 70, 92, 117, 145, ...

It can be seen that P4 + P7 = 22 + 70 = 92 = P8. However, their difference, 70 − 22 = 48, is not
pentagonal.

Find the pair of pentagonal numbers, Pj and Pk, for which their sum and difference are pentagonal
and D = |Pk − Pj| is minimised; what is the value of D?

*/

use std::time::Instant;
use std::cmp::min;


fn generate_pents(i: u64) -> Vec<u64> {
    // Generate the first i pentagonal numbers
    let mut v: Vec<u64> = vec![];
    for i in 1..i+1 {
        v.push(i * (3 * i - 1) / 2)
    }
    v
}

pub fn main() {
    let now = Instant::now();
    let pents = generate_pents(3000);
    let length = pents.len();
    let mut smallest = u64::max_value();
    for i in 1..length {
        for j in i+ 1..length{
            if pents.contains(&(pents[i] + pents[j])) && pents.contains(&(pents[j] - pents[i])) {
                println!("Sum {}, Diff {}", pents[i] + pents[j], pents[j] - pents[i]);
                smallest = min(smallest, pents[j] - pents[i]);
            }
        }
    }
    println!("{}", smallest);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}