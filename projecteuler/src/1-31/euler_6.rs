/*
The sum of the squares of the first ten natural numbers is,
12+22+...+102=385

The square of the sum of the first ten natural numbers is,
(1+2+...+10)2=552=3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025−385=2640

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

fn sum_of_squares(limit: u64) -> u64 {
    let mut total: u64 = 0;
    for i in 1..limit {
        total += i*i;
    }
    total
}

fn square_of_sums(limit: u64) -> u64 {
    let mut total = 0;
    for i in 1..limit {
        total += i;
    }
    return total*total
}

pub fn main() {
    assert_eq!(sum_of_squares(11), 385);
    assert_eq!(square_of_sums(11), 3025);

    println!("{}", square_of_sums(101) - sum_of_squares(101));
}