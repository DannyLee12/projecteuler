fn fibonacci(range: i32) -> i32 {
    if range == 0 || range == 1 { return 1 }
    return fibonacci(range - 1) + fibonacci(range - 2)
}

pub fn main() {
    let mut total = 0;
    for x in 0..100 {
        let fib = fibonacci(x);
//        println!("{}", fib);
        if fib > 4000000 { break }
        else if fib % 2 == 0 { total += fib };
    }
    println!("{}", total);
}