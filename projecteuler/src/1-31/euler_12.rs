/*
The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:

1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

Let us list the factors of the first seven triangle numbers:

     1: 1
     3: 1,3
     6: 1,2,3,6
    10: 1,2,5,10
    15: 1,3,5,15
    21: 1,3,7,21
    28: 1,2,4,7,14,28

We can see that 28 is the first triangle number to have over five divisors.

What is the value of the first triangle number to have over five hundred divisors?
*/
use std::time::Instant;

fn generate_triangles(range: u64) -> Vec<u64> {
    // Generate range triangular numbers
    let mut tri: Vec<u64> = vec![];
    for i in 1..range+1 {
        if i != 1 {
            tri.push(i + tri[(i-2) as usize])
        }
        else { tri.push(i) }
    }
    tri
}

pub fn main() {
    let now = Instant::now();
    // Generating triangles is 0(N) time complexity, therefore not a big issue to regen the
    // numbers, in any case, start with 10000 and scale in 10x
    let triangles = generate_triangles(1000000);
    let mut break_clause = false;
    for triangle in triangles {
        if break_clause { break };
        let mut divisors: i32 = 0;
        if triangle < 75_576_500 { continue };
        for i in 1..(triangle as f64).sqrt() as u64 {
            if triangle % i == 0 { divisors += 2 }
            if divisors >= 500 {
                println!{"{}. {}", triangle, divisors };
                break_clause = true;
                break;
                }
            }
        }
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}