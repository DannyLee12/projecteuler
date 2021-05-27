
use std::time::Instant;
/*
7,13,x,x,59,x,31,19
*/

pub fn main() {
    let now = Instant::now();
    let lean_list_i: Vec<u128> = vec![0, 1, 4, 6, 7];
    let lean_list_val: Vec<u128> = vec![7, 13, 59, 31, 19];
    let mut ts: u128 = 59 - 4;
    let mut counter: u128 = 10000 * 2;
    loop {
        let mut valid = true;
        for (x, y) in lean_list_i.iter().zip(&lean_list_val) {
            if (&ts + x) % y != 0 {
                valid = false;
                ts += 59;
                break
            }
        }
        if valid {
            println!("ts is {}", ts);
            println!("Script took: {} seconds", now.elapsed().as_secs());
            return
        }
        else if ts > counter {
                println!("Current value: {}", ts);
                counter += 10000;
            }
        }
    }