// Self powers
// Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.

extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigUint};
use num_traits::pow;

fn sum_self_powers(limit: usize) -> String {

    let mut n: usize = 1;
    let mut p: BigUint;
    let mut sum: BigUint = 0u32.into();
    
    loop {
        p = pow::pow(n.into(), n);
        sum += p;
        if n == limit { break; }
        n += 1;
    }

    let last_10: BigUint = sum % 10000000000u128;
    return last_10.to_string();
}

fn main() {
    println!("{}", sum_self_powers(1000));
}
