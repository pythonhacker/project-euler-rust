// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

mod common;
use common::{find_gcd, find_lcm};

fn main() {

    let numbers: Vec<u64>  = (2..21).collect();
    println!("{:?}", &numbers);
    
    println!("{}", find_gcd(&numbers));
    println!("{}", find_lcm(&numbers));

}
