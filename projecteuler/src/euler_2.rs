/*

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

fn sieve_erat() -> Vec<u64> {
    let mut v = vec![];
    for i in 2..10 {
        for j in 2..3175 {
            let val: u64 = i*j;
            if val > 775146 { break };
            if !v.contains( & val){
                v.push( val)
            }
//        println!("{}", val);
        }
    }
    println!("{}", v.len());
    let biggest = v.iter().max();
    println!("{:?}", v.iter().max());
    v
}

pub fn main() {
    let mut goal: u64 = 600851475143u64;
//    let mut goal: u64 = 8462696833u64;
//    let mut goal: u64 = 10086647u64;
    let mut goal: u64 = 408464633;
//    let mut largest: u64;
    let sieve = sieve_erat();

    for i in 3..4000 {
        if !sieve.contains(&i) && goal % i == 0 {
            println!("{}", i);
        }
    }
}
