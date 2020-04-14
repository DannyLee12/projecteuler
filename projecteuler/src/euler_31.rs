/*
In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:

    1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).

It is possible to make £2 in the following way:
    1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
How many different ways can £2 be made using any number of coins?
*/

use std::time::Instant;


pub fn main() {
    let now = Instant::now();
    let mut v:Vec<Vec<i32>> = vec![];
    let mut count = 0;

    for a in 0..2 {
        for b in 0..3 {
            for c in 0..6 {
                for d in 0..11 {
                    for e in 0..21 {
                        for f in 0..41 {
                            for g in 0..101 {
                                for h in 0..201 {
                                    if 200*a + 100*b + 50*c + 20*d + 10*e + 5*f + 2*g + 1*h == 200 {
                                        let row = vec![a, b, c, d, e, f, g, h];
                                        if ! v.contains(&row) {
                                            v.push(vec![a, b, c, d, e, f, g, h]);
                                            count += 1;
                                        }
//                                        println!("{:?}", v);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Total is: {}", count);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}
