// Sum of digits in factorial of 100

extern crate num_bigint;
use num_bigint::{BigUint};

fn factorial(n: u32) -> BigUint {

    let mut prod: BigUint = 1u32.into();
    
    for x in 2..n+1 {
        prod *= x;
    }

    return prod;    
}

fn factorial_sum(n: BigUint) -> u32 {

    let sum = n.to_string().chars().map(|d| d.to_digit(10)).fold(0, |sum, x| sum + x.unwrap());
    return sum;
}

fn main() {
    println!("{}", factorial_sum(factorial(100)));
}
