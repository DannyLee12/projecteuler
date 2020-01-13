/*

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

fn sieve_erat() -> Vec<u64> {
    let mut v = vec![];
    let range = 150010;
    for i in 2..50000000u64 {
        if i * i > range { break };
        for j in (i*i..range).step_by(i as usize) {
            if !v.contains( & j){
                v.push( j)
            }
//        println!("{}", val);
        }
    }
    println!("{}", v.len());
//    println!("{:?}", v.iter().nth(10001));
    v
}

pub fn main() {
    let sieve = sieve_erat();
    let mut count = 0;
    for i in 2..300000000u64 {
        if !sieve.contains(&i) {
            println!("{}", i);
            count += 1;
        }
        if count == 10001 {
            println!("{}", i);
            break;
        }
    }

}
