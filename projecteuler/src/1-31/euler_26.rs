/*
A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:
    1/2	= 	0.5
    1/3	= 	0.(3)
    1/4	= 	0.25
    1/5	= 	0.2
    1/6	= 	0.1(6)
    1/7	= 	0.(142857)
    1/8	= 	0.125
    1/9	= 	0.(1)
    1/10	= 	0.1

Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.
Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
*/

pub fn main() {
    for j in 7..1000_u128 {
        let s = (100000000000000000000000000000000000000/j).to_string();
//        let s = (340282366920938463463374607431768211455/j).to_string();
        let mut i = 30;
        let mut smallest = 0;
        let mut counter = 0;
        loop {
            let s1: String = s.chars().skip(0).take(i-7).collect();
            let s2: String = s.chars().skip(i).take(i-7).collect();
            if s1 == s2 {
                smallest = i;
                counter = j;
            }
            if j==43 {
                println!("{}, {}, {}", s, s1, s2)
            }
            i -= 1;
            if i < 10 { break }
        }
        if smallest > 18 {
            println!("{}: {}", counter, smallest);
        }
    }
}