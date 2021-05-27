/*
It can be seen that the number, 125874, and its double, 251748, contain exactly the same digits, but
in a different order.

Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.

*/

use std::time::Instant;

pub fn main() {
    let now = Instant::now();
    for x in 100000..1000000 {
        let digits: Vec<char> = x.to_string().chars().collect();
        let mut valid = true;
        for i in 2..7 {
            let newdigits: Vec<char> = (x * i).to_string().chars().collect();
            for nd in newdigits {
                if !digits.contains(&nd) {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            println!("{}, {}, {}, {}, {}, {}", x, x*2, x*3, x*4, x*5, x* 6);
            break;
        }
    }
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}