/*


By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.

   3
  7 4
 2 4 6
8 5 9 3

That is, 3 + 7 + 4 + 9 = 23.

Find the maximum total from top to bottom of the triangle below:

                75
               95 64
             17 47 82
            18 35 87 10
           20 04 82 47 65
          19 01 23 75 03 34
         88 02 77 73 07 63 67
        99 65 04 28 06 16 70 92
       41 41 26 56 83 40 80 70 33
      41 48 72 33 47 32 37 16 94 29
     53 71 44 65 25 43 91 52 97 51 14
    70 11 33 28 77 73 17 78 39 68 17 57
   91 71 52 38 17 14 91 43 58 50 27 29 48
  63 66 04 68 89 53 67 30 73 16 69 87 40 31
 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23

NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route. However, Problem 67, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! ;o)

*/

use std::time::Instant;
use std::fs;
use std::cmp::max;

pub fn main() {
    let now = Instant::now();
    let tree = read_tree();
    let mut sum_row: Vec<i32> = vec![0; 16];
    // Start at the last row
    let mut row = 13;
    let mut newvec = tree[14].to_vec();
    while row > 0 {
        // Create a new row with the max possible value at each position
        for (index, _value) in tree[row].iter().enumerate() {
            println!("newvec[index] {} & +1: {}", newvec[index], newvec[index+1]);
            println!("tree {}", tree[row][index]);
            sum_row[index] = tree[row][index] + max(newvec[index], newvec[index+1]);
        }
        println!("{:?}", sum_row);
        newvec = sum_row.to_vec();
        row -= 1
    }
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}

fn read_tree() -> Vec<Vec<i32>> {
    let tree = fs::read_to_string("src/tree.txt").expect("Unable to read file");
    let mut index = 0;
    let mut _total = 0;
    let mut v: Vec<Vec<i32>> = vec![vec![]; 15];
    for row in tree.split("\n") {
        for number in row.split(" ") {
            v[index].push(number.parse::<i32>().unwrap());
        }
        index += 1;
    }
    v
}
