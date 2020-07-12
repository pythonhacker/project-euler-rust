mod common;
use common::{is_prime_cached};

use std::collections::HashMap;

// Return primes percentage for a given value of square side = n
// NOTE: The performance of this can be tremendously improved
// by converting this to a prime count function and incrementally
// analyzing just the newly added numbers for the current square.

fn prime_percentage(n: u64, cache: &mut HashMap<u64, bool>) -> f64 {

    let diag_len: u64 = n/2 + 1;
//    println!("Half diagonal length is {}", diag_len);

    // Create squares of numbers from 1.. side_len
    let mut squares: Vec<u64> = vec![];

    for i in 0u64..diag_len {
        let odd_num = 2*i + 1;
        squares.push(odd_num*odd_num);
    }

    // Now create numbers across the diagonals
    let mut idx = 0;
    let mut prime_count = 0;
    
    for num in squares.iter() {
        let dnum1: u64 = *num - idx*2;
        let dnum2: u64 = *num - idx*4;
        let dnum3: u64 = *num - idx*6;

//        println!("{} {} {}", dnum1, dnum2, dnum3);
        if idx > 0 {
            for i in [dnum1, dnum2, dnum3].iter() {
                if is_prime_cached(*i, cache) {
                    prime_count += 1;
                }
            }
        }
        idx += 1;
    }

    let diag_length: f64 = (2*n - 1) as f64;
    let percent: f64 = 100.0*(prime_count as f64)/diag_length;
    
    return percent;
}

// At what n, where n=>side length of square spiral does
// ratio of primes along both diagonals falls below limit
// for the first time ?
fn primes_n_limit(limit: f64) -> u64 {

    // You get this by trial and error
    let mut n:u64 = 25001;
    let mut cache: HashMap<u64, bool> = HashMap::new();
    
    loop {
        let percent: f64 = prime_percentage(n, &mut cache);
        if percent < limit {
            println!("Limit {} breaks at n = {}", percent, n);
            break;
        } else {
            println!("Primes percent: {} at {}", percent, n);
        }
        n += 2;
    }

    return n;
}

fn main() {

    primes_n_limit(10.0);
}
