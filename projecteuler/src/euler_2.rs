/*

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/


// If none of the numbers up to number/2 are divisible
// Then the number is prime
fn prime_numbers(number: i32) -> bool {
    for x in 2..(number/2 + 1) {
        if number % x == 0 {return false}
    }
    true
}

pub fn main() {
    assert!(prime_numbers(3));
    assert_eq!(prime_numbers(4), false);
    assert_eq!(prime_numbers(5), true);

}