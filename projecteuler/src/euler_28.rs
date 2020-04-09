/*
Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:

21 22 23 24 25
20  7  8  9 10
19  6  1  2 11
18  5  4  3 12
17 16 15 14 13

It can be verified that the sum of the numbers on the diagonals is 101.

What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

*/

use std::time::Instant;

pub fn main() {
    let now = Instant::now();
    // Test case
    let size = 1001;
    let mut diagonal_1 = 3;
    let mut diagonal_2 = 5;
    let mut diagonal_3 = 7;
    let mut diagonal_4 = 9;
    let mut number_to_add = 0;
    let mut total = 1 + 3 + 5 + 7 + 9;
    while diagonal_4 < size * size {
        diagonal_1 += 10 + number_to_add;
        diagonal_2 += 12 + number_to_add;
        diagonal_3 += 14 + number_to_add;
        diagonal_4 += 16 + number_to_add;
        total += diagonal_1 + diagonal_2 + diagonal_3 + diagonal_4;
        number_to_add += 8;
    }
    println!("Total is {}", total);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}
