/*
The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the digits 0 to 9 in some order, but it also has a rather interesting sub-string divisibility property.

Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:

    d2d3d4=406 is divisible by 2
    d3d4d5=063 is divisible by 3
    d4d5d6=635 is divisible by 5
    d5d6d7=357 is divisible by 7
    d6d7d8=572 is divisible by 11
    d7d8d9=728 is divisible by 13
    d8d9d10=289 is divisible by 17

Find the sum of all 0 to 9 pandigital numbers with this property.
*/

use std::time::Instant;
use itertools::Itertools;

fn has_property(perm: String) -> u32 {
//    println!("{}", perm);
    let mut v: Vec<u32> = vec![2, 3, 5, 7, 11, 13, 17];
    let mut has = true;
    for (i, d) in (2..9).zip(v) {
        let num: String = perm.chars().skip(i-1).take(3).collect();
        if num.parse::<u32>().unwrap() % d != 0  {
            has = false;
            break;
        }
    }
    if has {
        return perm.parse::<u32>().unwrap()
    }
    0
}

pub fn main() {
    let now = Instant::now();
    let items = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut perms: Vec<String> = vec![];
    let mut total: u64 = 0;
    let length = items.len();
    for perm in items.iter().permutations(length).unique() {
        let s: String = perm.into_iter().collect();
        total += has_property(s) as u64;
    }
    println!("{}", total);
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}