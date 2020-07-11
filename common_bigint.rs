// Common functions using BigUint type

extern crate num_bigint;
use num_bigint::{BigUint};

pub fn factorial(n: u32)->BigUint {

    let mut fact: BigUint = 1u32.into();
    
    for i in 2..n+1 {
        fact *= i;
    }

    return fact;
}

pub fn n_cr(n: u32, r: u32)->BigUint {

    let nfact: BigUint = factorial(n);
    let rfact: BigUint = factorial(r);
    let rest_fact: BigUint = factorial(n-r);

    return nfact/(rfact*rest_fact);

}
