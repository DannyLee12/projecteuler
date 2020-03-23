mod euler_27;
mod euler_7;

fn main() {
    let sieve = euler_7::sieve_erat(200000);
    euler_27::main(sieve);
}
