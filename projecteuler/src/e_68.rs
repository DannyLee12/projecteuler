/*
https://projecteuler.net/problem=68
*/

use std::time::Instant;


pub fn main() {
    let now = Instant::now();

    for a in 1..11 {
        for b in 1..11 {
            if a == b { continue }
            for c in 1..11 {
                if b == c {continue}
                    if a + b + c == 14 {
                        // v1
                        let mut vals = vec![a, b, c];
                        let counter = step(b, &mut vals, 0, b);
                        if counter > 3 {
                            println!("{:?}", vals,);
                        }
                    }
            }
        }
    }
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());

}

fn step(c: i32, vals: &mut Vec<i32>, counter: i32, b: i32) -> i32 {
    for d in 1..11 {
        if vals.contains(&d) { continue }
        else if counter == 3 {
//            println!("Inner pre: {:?}, {}, {}, {}", vals, d, c, b);
            if d + c + b == 14 {
                vals.push(d);
                println!("Inner: {:?}, {}", vals, counter);

            }
        }
        else {
            for e in 1..11 {
               if vals.contains(&e) || e == d { continue }
                    if d + c + e == 14 {
                    // v2
                    vals.push(d);
                    vals.push(e);
//                    println!("{:?} {}", vals, counter);
                    return 1 + step(e, vals, counter + 1, b);
            }
        }
        }

    }
    0
}