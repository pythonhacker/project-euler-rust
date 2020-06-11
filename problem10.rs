// Find the sum of all the primes below two million.
mod common;

use common::{is_prime_i64_ex};

fn prime_sum(limit: i64) -> i64 {

    let (mut n, mut sum)=(2,0);
    let mut primes:Vec<i64> = vec![2];
    while n<limit {
        if is_prime_i64_ex(n, &primes) {
            sum += n;
            primes.push(n);
        }

        n += 1;
    }
        
    return sum;
}

fn main() {
    println!("{}", prime_sum(2000000));
}
