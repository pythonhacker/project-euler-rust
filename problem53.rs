// How many values of nCr where n <= 100 are greater than 1 million ?
extern crate num_bigint;
use num_bigint::{BigUint};

mod common_bigint;
use common_bigint::{n_cr};

fn main() {

    let limit: BigUint = 1000000u32.into();
    let mut count: usize = 0;
    
    for n in 23..101 {
        // 100*99*98 < 1million so we need to go upto n-3 only
        for r in 2..n-3 {
            let n_cr = n_cr(n, r);
            if n_cr > limit {
                println!("{}C{} => {} exceeds 1 million", n, r, n_cr);
                count += 1;
            }
        }
    }

    println!("count is {}", count);

}
