// Maximum digital sum of powers

extern crate num_bigint;
use num_bigint::{BigUint};
extern crate num_traits;

mod common_bigint;

use common_bigint::{power, digit_to_vector};

fn main() {

    let mut num_vec: Vec<BigUint>;
    let mut num_pow;
    let mut max_sum: BigUint = 0u32.into();
    let (mut max_a, mut max_b) = (0u32, 0u32);
    
    for a in 2u32..100 {
        for b in 2..100 {
            num_pow = power(a.into(), b);
            num_vec = digit_to_vector(num_pow.to_owned());
            // Sum
            let sum: BigUint = num_vec.iter().fold(0u32.into(), |s, x| s+x);
            if sum > max_sum {
                max_a = a; max_b = b as u32; max_sum = sum;
            }
        }
    }

    println!("Max sum:{} for {}^{}", max_sum, max_a, max_b);
}
