/*
The nth term of the sequence of triangle numbers is given by, tn = ½n(n+1); so the first ten
triangle numbers are:

1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

By converting each letter in a word to a number corresponding to its alphabetical position and
adding these values we form a word value. For example, the word value for SKY is 19 + 11 + 25 = 55
= t10. If the word value is a triangle number then we shall call the word a triangle word.

Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly
two-thousand common English words, how many are triangle words?
*/

use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn triangles(n: u8) -> HashMap<char, u64> {
    let mut hm: HashMap<char, u64> = HashMap::new();
    for i in 1..n + 1{
        hm.insert((65 + i - 1) as char , i as u64);
    }
    hm.insert('"', 0);
    hm
}

fn triangles_vec(n: u8) -> Vec<u64> {
    let mut v: Vec<u64> = vec![];
    for i in 1..n+1 {
        v.push((i as u64 * (i as u64 + 1)) / 2);
    }
    v
}

pub fn main() {
    let now = Instant::now();
    let mut total = 0;
    let triangles = triangles(26);
    println!("{:?}", triangles);
    let triangles_vec = triangles_vec(26);
    let data = fs::read_to_string("/Users/daniellee/Development/projecteuler/projecteuler/src/words.txt").expect("Unable to read file");
    let wordlist: Vec<&str> = data.split(",").collect();
    for word in wordlist {
        let mut word_total = 0;
        for c in word.chars() {
//            println!("{}, {}", c, triangles[&c]);
            word_total += triangles[&c];
//            println!("{}, {}, {}", c, triangles[&c], word_total);
        }
        if triangles_vec.contains(&word_total) {
            println!("{}", word);
            total += 1;
        }
    }
    println!("Total triangle words: {}", total);
    println!("Script took {} seconds to run", now.elapsed().as_millis());
}