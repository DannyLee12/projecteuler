/*

A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

fn is_palindrome(number: &str) -> bool {
    return number == number.chars().rev().collect::<String>()
}

pub fn main() {
    assert_eq!(is_palindrome("thissiht"), true);
    assert_eq!(is_palindrome("notnot"), false);
    let mut max: u64 = 0;
    for i in 0..1000 {
        for j in 0..1000{
            let mut value: u64 = (1000 - i) * (1000 - j);
            if is_palindrome(&value.to_string()) {
                if value > max {
                    max = value
                }
                println!{"{}", max};
            }
        }
    }
}
