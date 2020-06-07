// Find the sum of all the primes below two million.

fn is_prime(number: i64, primes: &Vec<i64>) -> bool {
    let mut flag = true;

    let x = (number as f64).sqrt() as i64 + 1;
        
    for item in primes.iter() {
        if *item >= x { break; }
        if number % *item == 0 {
            flag = false;
            break;
        }
    }

    return flag;
}

fn prime_sum(limit: i64) -> i64 {

    let (mut n, mut sum)=(2,0);
    let mut primes:Vec<i64> = vec![2];
    while n<limit {
        if is_prime(n, &primes) {
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
