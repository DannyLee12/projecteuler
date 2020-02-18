/*
Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.
For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.
What is the total of all the name scores in the file?
*/
use std::time::Instant;
use std::fs::read_to_string;
use std::collections::HashMap;

pub fn main() {
    let now = Instant::now();
    let data = read_to_string("src/names.txt").expect("Unable to read file");
    let mut v = vec![];
    for name in data.split(","){
        v.push(name.replace("\"", ""));
    }
    v.sort();
    let mut total: u128 = 0;
    for (i, word) in v.iter().enumerate() {
        let mut word_total = 0;
        for c in word.chars(){
            word_total += (c as u128 - 64);
        }
        total += (i as u128 + 1) * word_total;
    }
    println!("{}", total);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}
