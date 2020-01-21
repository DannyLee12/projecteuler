/*
215 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?
*/

pub fn main() {
    let mut value: f64 = 2.0;
    for _ in 0..1000 {
        value *= 2_f64
    }
    println!("{}", value);
    let mut total: u64 = 0;
    for c in value.to_string().chars() {
        println!("{}", c);
        total += c.to_string().parse::<u64>().unwrap();
        println!("{}", total);
    }
    println!("{}", total)
}