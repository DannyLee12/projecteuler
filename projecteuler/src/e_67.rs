/*
By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.

3
7 4
2 4 6
8 5 9 3

That is, 3 + 7 + 4 + 9 = 23.

Find the maximum total from top to bottom in triangle.txt (right click and 'Save Link/Target As...'), a 15K text file containing a triangle with one-hundred rows.
*/

use std::time::Instant;
use std::cmp::max;
use std::fs;

pub fn main() {
    let now = Instant::now();
    let tree = read_tree();
    let mut sum_row: Vec<i32> = vec![0; 101];
    // Start at the last row
    let mut row = 98;
    let mut newvec = tree[99].to_vec();
    while row > 0 {
        // Create a new row with the max possible value at each position
        for (index, _value) in tree[row].iter().enumerate() {
            sum_row[index] = tree[row][index] + max(newvec[index], newvec[index+1]);
        }
        newvec = sum_row.to_vec();
        row -= 1
    }
    println!("Script took {} seconds to run", now.elapsed().as_millis());
    println!("Largest range: {}", max(sum_row[0], sum_row[1]) + tree[row][0])
}


fn read_tree() -> Vec<Vec<i32>> {
    let tree = fs::read_to_string("src/p067_triangle.txt").expect("Unable to read file");
    let mut index = 0;
    let mut _total = 0;
    let mut v: Vec<Vec<i32>> = vec![vec![]; 100];
    for row in tree.split("\n") {
        for number in row.split(" ") {
            v[index].push(number.parse::<i32>().unwrap());
        }
        index += 1;
    }
    v
}
