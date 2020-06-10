// What is the 10001st prime number?

fn nth_prime(limit:i32)->i64 {

    let (mut n, mut count) = (2, 1);
    let mut primes:Vec<i64> = vec![2];
    let mut is_prime:bool;
    
    while count<limit {
        n += 1;
        is_prime = true;

        for x in primes.iter() {
            if n % x == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(n);
            count += 1;
        }
    }

    return n;
}

fn main() {
    
    println!("{}", nth_prime(10001));
}
