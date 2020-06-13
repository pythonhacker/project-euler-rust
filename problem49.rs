// Prime permutations

extern crate itertools;

use itertools::Itertools;

mod common;
use common::{vector_to_digit_u32, digit_to_vector_u32, is_prime_u32};

fn prime_permute() -> String {

    let mut n:u32 = 1001;
    let mut count: usize = 0;
    
    loop {
        if is_prime_u32(n) {
            let n_vec: Vec<u32> = digit_to_vector_u32(n);
            let mut n_prime: Vec<u32> = vec![n];
            let mut idx: usize = 0;
            
            for it in n_vec.iter().permutations(4) {
                if *it[0] == 0 { continue; }

                let mut n_perm: u32 = vector_to_digit_u32(it.to_vec());
                
                if is_prime_u32(n_perm) && (n_perm > n_prime[idx]) && ((n_perm - n_prime[idx]) == 3330) {
                    n_prime.push(n_perm);
                    idx += 1;
                }
            }

            if n_prime.len() == 3 {
                count += 1;
                println!("{:?}", n_prime);

                if count == 2 {
                    return n_prime.iter().map( |x| x.to_string() ).join("");
                }
            }
        }

        n += 1;
        if n == 9999 { break; }
    }

    return "".to_string();
}

fn main() {
    println!("{}", prime_permute());
}
