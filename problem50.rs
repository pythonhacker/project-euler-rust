// Consecutive prime sum
//  Which prime, below one-million, can be written as the sum of the most consecutive primes?

mod common;
use common::{is_prime, consecutive_primes};

fn max_consecutive_prime_sum(limit: u64) {

    // We need just primes till maximum half of the limit
    let primes: Vec<u64> = consecutive_primes(limit/2);
    
    let mut idx: usize = 0;
    let mut sum: u64;
    let mut max_sum: u64 = 0;
    let mut max_size: usize = 0;
    
    // println!("{:?}", primes);

    loop {
        let mut count: usize = 0;
        let mut size: usize = 0;

        sum = 0;
        println!("Iteration {}", idx+1);
        
        for n in primes.iter() {
            if count < idx { count += 1; continue; }

            sum += n;
            if sum > limit { break; }

            size += 1;
            
            if is_prime(sum) && sum > max_sum  && sum < limit && size > max_size {
                max_sum = sum;
                max_size = size;
            }
        }

        println!("Max sum => {} {} {}", max_sum, max_size, primes[idx]);
        
        // No point continuing if the next prime + current_sum > limit
        if primes[idx] + max_sum > limit {
            break;
        }
        
        idx += 1;
        if idx == primes.len() { break; }

    }

    println!("Final max sum => {}", max_sum);
}

fn main() {
    max_consecutive_prime_sum(1000000);
}
