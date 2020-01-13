/*
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

fn is_divisible(number: u64) -> bool {
    for i in 1..20 {
        if number % i != 0 { return false }
    }
    return true
}

pub fn main() {
    for x in 1..300000000u64 {
        if is_divisible(x) {
            println!("{}", x);
            break;
        }
    }
}