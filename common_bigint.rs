// Common functions using BigUint type

extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigUint};
use self::num_traits::pow;

use std::iter::FromIterator;

// Factorial using BigUint
#[allow(dead_code)]        
pub fn factorial(n: u32)->BigUint {

    let mut fact: BigUint = 1u32.into();
    
    for i in 2..n+1 {
        fact *= i;
    }

    return fact;
}

// nCr (r combinations of n items) using BigUint
#[allow(dead_code)]        
pub fn n_cr(n: u32, r: u32)->BigUint {

    let nfact: BigUint = factorial(n);
    let rfact: BigUint = factorial(r);
    let rest_fact: BigUint = factorial(n-r);

    return nfact/(rfact*rest_fact);

}

#[allow(dead_code)]
// is_palindrome using BigUint 
pub fn is_palindrome(number: BigUint) -> bool {

    let num_s: String = number.to_string();
    let mut num_chars: Vec<char> = num_s.chars().collect();

    num_chars.reverse();

    let num_s_rev: String = String::from_iter(num_chars);

    return num_s == num_s_rev;
}

#[allow(dead_code)]
// digit_to_vector using BigUint 
pub fn digit_to_vector(num: BigUint) -> Vec<BigUint> {

    let mut vdigits: Vec<BigUint> = vec![];
    let mut n = num;
    let start: BigUint = 0u32.into();
    let p: BigUint = 10u32.into();
    let q: BigUint = 10u32.into();
    
    while n > start {
        // Jumping thru ownership hoops since BigUint does not implement Copy trait :-|
        vdigits.push(n.to_owned() % p.to_owned());
        n = n / q.to_owned();
    }

    vdigits.reverse();
    
    return vdigits;
}

#[allow(dead_code)]
// vector_to_digit using BigUint 
pub fn vector_to_digit(num_vec: &Vec<BigUint>) -> BigUint {

    let mut num: BigUint = 0u32.into();
    let mut i: usize = 0;
    let pow_base: BigUint = 10u32.into();
    
    for n in num_vec.iter().rev() {
        // Jumping thru ownership hoops since BigUint does not implement Copy trait :-|        
        num += pow::pow(pow_base.to_owned(), i)*n;
        i += 1;
    }

    return num;
}
