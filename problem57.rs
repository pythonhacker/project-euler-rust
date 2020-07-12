// Square root convergents
// In the first one-thousand expansions of sqrt(2) as a continued fraction, how many fractions
// contain a numerator with more digits than the denominator?

extern crate num_bigint;
use num_bigint::{BigUint};

use std::collections::HashMap;

fn continued_fraction(n_iter: u32, cache: &mut HashMap<u32, (BigUint, BigUint)>) -> (BigUint, BigUint) {

    // Without this cache, the solution will draaag.
    if cache.contains_key(&n_iter) {
        // println!("Hit cache => {}", n_iter);
        let val = cache.get(&n_iter).unwrap().to_owned();

        return val;
    }
    
    if n_iter == 0 {
        return (1u32.into(), 1u32.into());
    }
    if n_iter == 1 {
        return (3u32.into(), 2u32.into());
    } else {
        let last = continued_fraction(n_iter - 1, cache);
        let last_before = continued_fraction(n_iter - 2, cache);

        let last_num = last.0 - last.1.to_owned();
        let last_denom = last.1;
        let last_before_num = last_before.0 - last_before.1;
        
        let num: BigUint = last_num.to_owned()*2u32 + last_before_num;
        let denom: BigUint = last_denom*2u32 + last_num.to_owned();

        let val = (num + denom.to_owned(), denom);
        cache.insert(n_iter, val.to_owned());

        return val;
    }
        
}

fn main() {
    let mut cache: HashMap<u32, (BigUint, BigUint)> = HashMap::new();
    let mut count: usize = 0;
        
    for i in 1..1001 {
        let fract = continued_fraction(i, &mut cache);
        let num_s = fract.0.to_string();
        let denom_s = fract.1.to_string();

        if num_s.len() > denom_s.len() {
            count += 1;
            println!("Iteration #{} is a hit", i);
            // println!("{} {}", fract.0, fract.1);
        }
    }

    println!("{}", count);

}
