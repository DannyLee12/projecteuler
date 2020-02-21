/*
A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:
012   021   102   120   201   210
What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
*/
use itertools::Itertools;
use std::time::Instant;

pub fn main() {
    let now = Instant::now();
    let items = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut perms: Vec<String> = vec![];
    let mut total = 0;
    for perm in items.iter().permutations(items.len()).unique() {
        perms.push(perm.into_iter().collect());
        total += 1;
        if total == 5000000 {
            break;
        }
     }
    perms.sort();
    println!("{}", perms[1000000 - 1]);
    println!("{}", perms[1000000]);
    println!("{}", perms[1000000 + 1]);

    println!("Script took {} seconds to run", now.elapsed().as_secs());
}