/*
If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.

Not all numbers produce palindromes so quickly. For example,

349 + 943 = 1292,
1292 + 2921 = 4213
4213 + 3124 = 7337

That is, 349 took three iterations to arrive at a palindrome.

Although no one has proved it yet, it is thought that some numbers, like 196, never produce a
palindrome. A number that never forms a palindrome through the reverse and add process is called a
Lychrel number. Due to the theoretical nature of these numbers, and for the purpose of this problem,
we shall assume that a number is Lychrel until proven otherwise. In addition you are given that for
every number below ten-thousand, it will either (i) become a palindrome in less than fifty
iterations, or, (ii) no one, with all the computing power that exists, has managed so far to map it
to a palindrome. In fact, 10677 is the first number to be shown to require over fifty iterations
before producing a palindrome: 4668731596684224866951378664 (53 iterations, 28-digits).

Surprisingly, there are palindromic numbers that are themselves Lychrel numbers; the first example
is 4994.

How many Lychrel numbers are there below ten-thousand?

NOTE: Wording was modified slightly on 24 April 2007 to emphasise the theoretical nature of Lychrel
numbers.
*/

use std::time::Instant;

fn is_palindrome(num: &str) -> bool {
    let n = num.chars().count();
    let num2: &str = &num[n/2..n].chars().rev().collect::<String>()[..];
    for (c1, c2) in num[0..n / 2].chars().zip(num2.chars()){
        if c1 != c2 {
            return false
        }
    }
    true
}

pub fn main() {
    let now = Instant::now();
    let mut lychels = 0;
    for mut i in 1..10000u128 {
        let mut iteration = 1;
        loop {
            if iteration >= 50 {
                lychels += 1;
                break
            }
            let rev: &str = &(i.to_string().chars().rev().collect::<String>())[..];
            i += rev.parse::<u128>().unwrap();
            iteration += 1;
            if is_palindrome(&i.to_string()[..]) {
                break;
            }
        }
    }
    println!("Total Lychels: {}", lychels);
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}
