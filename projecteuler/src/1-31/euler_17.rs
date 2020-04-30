/*
If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?

NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.
*/
use std::collections::HashMap;

pub fn main() {
    let mut hp: HashMap<i32, String> = HashMap::new();
    hp.insert(1, "one".to_string());
    hp.insert(2, "two".to_string());
    hp.insert(3, "three".to_string());
    hp.insert(4, "four".to_string());
    hp.insert(5, "five".to_string());
    hp.insert(6, "six".to_string());
    hp.insert(7, "seven".to_string());
    hp.insert(8, "eight".to_string());
    hp.insert(9, "nine".to_string());
    hp.insert(10, "ten".to_string());
    hp.insert(11, "eleven".to_string());
    hp.insert(12, "twelve".to_string());
    hp.insert(13, "thirteen".to_string());
    hp.insert(14, "fourteen".to_string());
    hp.insert(15, "fifteen".to_string());
    hp.insert(16, "sixteen".to_string());
    hp.insert(17, "seventeen".to_string());
    hp.insert(18, "eighteen".to_string());
    hp.insert(19, "nineteen".to_string());
    hp.insert(20, "twenty".to_string());
    hp.insert(30, "thirty".to_string());
    hp.insert(40, "forty".to_string());
    hp.insert(50, "fifty".to_string());
    hp.insert(60, "sixty".to_string());
    hp.insert(70, "seventy".to_string());
    hp.insert(80, "eighty".to_string());
    hp.insert(90, "ninety".to_string());
    hp.insert(1000, "onethousand".to_string());

    let mut total = 0;
    for i in 1..1001 {
        if ! hp.contains_key(&i) {
            if i < 100 {
//                println!("{}", i/10);
//                println!("{}", i%10);
                total += hp[&((i / 10)*10)].chars().count();
                total += hp[&(i % 10)].chars().count();
            }
            else if i > 99 {
                total += "hundred".to_string().chars().count();
                // Hundreds
                println!("{}", hp[&(i / 100)]);
                total += hp[&(i / 100)].chars().count();
                // Tens
                let tens = i % 100;
                if hp.contains_key( &tens) {
                    total += hp[&tens].chars().count();
                    total += "and".to_string().chars().count();
                }
                else if tens > 0 {
                    total += "and".to_string().chars().count();
                    total += hp[&((tens / 10) * 10)].chars().count();
                    // Units
                    total += hp[&(tens % 10)].chars().count();
                }
            }
        }
        else {
            total += hp[&i].chars().count();
        }
    }
    println!("{}", total)

}
