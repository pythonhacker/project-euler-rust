// What is the sum of the digits of the number 2**1000 ?

// This is a one-liner in Python and gets way too complicated in Rust.
// It is a big surprise that Rust being such a high performant language
// does not have support for Big integers in its own standard library.
// Forced to use 2 external crates... Shame.

extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigUint};
use num_traits::pow;

fn power_sum(n: u32, e: usize) {

    let big_n: BigUint = n.into();
    let big_a: BigUint = pow::pow(big_n, e);    
    println!("{}", big_a);

    let digits:Vec<u32> = big_a.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();    
    let sum = big_a.to_string().chars().map(|d| d.to_digit(10)).fold(0, |sum, x| sum + x.unwrap());//.unwrap()).collect();
    println!("{:?}", digits);
    println!("{}", sum);

}

fn main() {

    power_sum(2, 1000);
}
