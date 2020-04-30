/*

Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.

How many such routes are there through a 20×20 grid?

*/
use std::time::Instant;

pub fn main(grid_size: u128) -> i32 {
    let now = Instant::now();
    let mut v: Vec<i32> = vec![1; grid_size as usize];
    for i in 0..grid_size {
        println!("{}", i);
        for j in 0..i {
            if j == 0 { v[j as usize] = v[j as usize] + 1 }
            else { v[j as usize] = v[j as usize] + v[j as usize - 1] }
        }
        if i == 0 { v[i as usize] = 2 }
        else { v[i as usize] = 2 * v[i as usize - 1] }
        println!("{:?}", v);
    }
    println!("{}", v[grid_size as usize - 1]);
    println!("Script took {} seconds to run", now.elapsed().as_secs());
    v[grid_size as usize - 1]
}
