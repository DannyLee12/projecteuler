/*

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
a2 + b2 = c2

For example, 32 + 42 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

pub fn main() {
    let mut a = 1;
    let mut b = 1;
    let mut c: f64 = 1.0;
    for a in 1..1000 {
        for b in a..1000 {
            c = ((a*a + b*b) as f64).sqrt();
            if (c - (c as i32) as f64) < 0.0001 {
                if a + b + c as i32 == 1000 {
                    println!("a: {}", a);
                    println!("b: {}", b);
                    println!("c: {}", c);
                    println!("abc = {}", a*b*c as i32)
                }
            }
            else if a + b + c as i32 > 1000 { break }
        }
    }
}
