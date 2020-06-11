// Find the sum of the only eleven primes that are both truncatable from left to right and right to left.

mod common;

use common::{vector_to_digit, digit_to_vector, is_prime};

fn is_truncatable_prime(n: u64) -> bool {

    let n_vec = digit_to_vector(n);
    let mut n_slice_num: u64;
    
    for i in 1..n_vec.len() {
        let mut n_slice_left = &n_vec[i..];
        n_slice_num = vector_to_digit(&n_slice_left.to_vec());
        
        // println!("{}", n_slice_num);        
        if !is_prime(n_slice_num) {
            return false;
        }
    }

    for i in (1..n_vec.len()).rev() {
        let mut n_slice_right = &n_vec[..i];
        n_slice_num = vector_to_digit(&n_slice_right.to_vec());

        // println!("{}", n_slice_num);                
        if !is_prime(n_slice_num) {
            return false;
        }
    }    

    return true;
}

fn main() {

    let mut sum: u64 = 0;
    
    for x in 11..10000000 {
        if !is_prime(x) {continue;}
        if is_truncatable_prime(x) {
            println!("{}", x);
            sum += x;
        }
    }

    println!("Sum => {}", sum);
}
