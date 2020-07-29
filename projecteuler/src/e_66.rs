/*
Consider quadratic Diophantine equations of the form:

x2 – Dy2 = 1

For example, when D=13, the minimal solution in x is 6492 – 13×1802 = 1.

It can be assumed that there are no solutions in positive integers when D is square.

By finding minimal solutions in x for D = {2, 3, 5, 6, 7}, we obtain the following:

32 – 2×22 = 1
22 – 3×12 = 1
92 – 5×42 = 1
52 – 6×22 = 1
82 – 7×32 = 1

Hence, by considering minimal solutions in x for D ≤ 7, the largest x is obtained when D=5.

Find the value of D ≤ 1000 in minimal solutions of x for which the largest value of x is obtained.
*/

use std::time::Instant;
use std::cmp::max;

pub fn main() {
    let now = Instant::now();
    let mut max_x: u64 = 9;
    let mut b = false;
    let mut ds: Vec<i128> = vec![];
    for D in 2..1001 {
        let sq2 = (D as f64).sqrt();
        if sq2 - (sq2 as i64 as f64) == 0.0 { continue };
        println!("D: {}", D);
        let d = D as f64;
        for x in max_x..100000000000u64 {
        let sq = (((x as f64 * x as f64) - 1.0)/d).sqrt();
//        let sq = ((1 + D*y*y) as f64).sqrt();
        if sq - (sq as i64 as f64) == 0.0 {
            println!("D: {}, x: {}", D,  x);
            max_x = max(max_x, x);
            break;
        }
        }
//        let sq = (D as f64).sqrt();
//        if sq - (sq as i64 as f64) == 0.0 { continue };
//        for x in 1000000..5000001 {
//            for y in 100000..500001 {
//                let lhs = x*x - D*y*y;
//                if lhs == 1 {
//                    max_x = max(max_x, x);
//                    println!("D: {}, x: {}, y: {}", D, x, y);
//                    b = true;
//                    break;
//                }
//                else if lhs < 1 {
////                    println!("Break");
////                    println!("D: {}, x: {}, y: {}", D, x, y);
//                    break;
//                }
//            }
//            if b {
//                b = false;
//                break;
//            }
//        }
    }
    println!("Max x: {}", max_x);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}