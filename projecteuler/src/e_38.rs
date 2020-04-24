/*

If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are
exactly three solutions for p = 120.
{20,48,52}, {24,45,51}, {30,40,50}
For which value of p ≤ 1000, is the number of solutions maximised?
*/

use std::time::Instant;

pub fn main() {
    let now = Instant::now();
    let mut p_max = 0;
    let mut total_max = 0;
    for p in 1..1000u128 {
        let mut total = 0;
        for a in 1..p/2 {
            for b in a..p-1 {
                if (a + b) > p {continue};
                let c = p - (a + b);
                if a*a + b*b == c*c {
                    total += 1;
                }
            }
        }
        if total > total_max {
            println!("New max: {}", total);
            total_max = total;
            p_max = p;
        }
    }
    println!("Value maximixed for p = {} with {} values", p_max, total_max);

    println!("Script took {} seconds to run", now.elapsed().as_millis());
}