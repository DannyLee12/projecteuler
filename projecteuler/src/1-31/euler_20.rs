/*
n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!
*/
use num_bigint::BigUint;
use num_traits::{FromPrimitive, ToPrimitive};

fn factorial(value: BigUint) -> BigUint {
    let mut result: BigUint = FromPrimitive::;
    let mut v= 1;
    while value >= v as BigUint {
        result *= v as BigUint;
        println!("{}", v);
        v += 1;
    }
    result
}


pub fn main() {
//    assert_eq!(factorial(10), 3628800);
    println!("{}", factorial(19 as BigUint));
}