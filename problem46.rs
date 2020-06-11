// Goldbach's other conjecture
// What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?

mod common;
use common::{is_prime};

fn smallest_composite() {

    let mut n: u64 = 35;
    let mut flag: bool;
    
    loop {
        // println!("Trying {}", n);
        if !is_prime(n) { 
            flag = false;
            let lesser_primes: Vec<u64> = (2..n-1).filter(|x| is_prime(*x)).collect();
            
            for p in lesser_primes.iter() {
                let mut root: f64 =(((n - p) as f64)*0.5).sqrt();
                if root.trunc() == root {
                    flag = true;
                    break;
                }
            }

            if !flag {break;}
        }

        n += 2;
    }

    println!("{}", n);
}

fn main() {
    smallest_composite();
}
