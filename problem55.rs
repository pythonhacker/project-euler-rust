// Lychrel numbers
// Find count of Lychrel numbers < 10000

extern crate num_bigint;
use num_bigint::{BigUint};
extern crate num_traits;

mod common_bigint;
mod common;

use std::u32;
use common_bigint::{is_palindrome, digit_to_vector, vector_to_digit};

fn is_lychrel(n: BigUint, n_iter: &mut u32) -> bool {

    // Jumping thru ownership hoops since BigUint does not implement Copy trait :-|    
    let mut n_vector: Vec<BigUint> = digit_to_vector(n.to_owned());
    n_vector.reverse();

    let n_rev: BigUint = vector_to_digit(&n_vector);

    let n_sum: BigUint = n + n_rev;

    *n_iter += 1;
    if *n_iter == 50 {
        println!("Max iterations reached");
        return true;
    }

    // Jumping thru ownership hoops since BigUint does not implement Copy trait :-|    
    if is_palindrome(n_sum.to_owned()) {
        println!("palindrome at {} iterations, num: {}", n_iter, n_sum);
        return false;
    } else {
        println!("iterations: {}, num: {}", n_iter, n_sum);
        return is_lychrel(n_sum, n_iter);
    }

}

fn main() {
    let mut count = 0;
    let mut n:u32 = 1;
    let mut n_iter:u32;

    
    loop {
        println!("Trying {}", n);
        n_iter = 0;
        
        if is_lychrel(n.into(), &mut n_iter) {
            println!("IS LYCHREL => {}", n);
            count += 1;
        }

        n += 1;
        if n == 10000 { break; }
    }

    println!("Count {}", count);
}
