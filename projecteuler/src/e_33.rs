/*
The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to
simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling
the 9s.
We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
There are exactly four non-trivial examples of this type of fraction, less than one in value, and
containing two digits in the numerator and denominator.
If the product of these four fractions is given in its lowest common terms, find the value of the
denominator.
*/

use std::time::Instant;

pub fn main() {
    let now = Instant::now();
    for x in 10..100 {
        for y in 10..100 {
            let sx = x.to_string();
            let sy = y.to_string();
            let mut num = "".to_string();
            let mut dem = "".to_string();
            if x as f64 / y as f64 > 1.0 {continue};
            for i in 1..10 {
                let si = i.to_string();
                if sx.contains(&si) && sy.contains(&si) {
                    num = "".to_string();
                    dem = "".to_string();
                    for a in sx.chars() {
                        let sa = a.to_string();
                        if sa == si {
                            continue
                        }
                        num.push_str(&sa);
                    }
                    for b in sy.chars() {
                        let sb = b.to_string();
                        if sb == si {
                            continue
                        }
                        dem.push_str(&sb);
                    }
                    if num == "0".to_string() || dem == "0".to_string() || num == "".to_string() || dem == "".to_string()  { break }
                    if num == dem { break };
                    if (num.parse::<f64>().unwrap())/(dem.parse::<f64>().unwrap()) == (x as f64)/(y as f64) {
                        println!("Valid: {}/{}", sx, sy);
                    }
                }
            }
        }
    }
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}