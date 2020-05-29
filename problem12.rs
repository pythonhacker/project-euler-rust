// Highly divisible triangular number
// See the problem at https://projecteuler.net/problem=12

use std::collections::HashMap;

fn get_divisors(n:i64, divisors: &mut HashMap<i64, i64>) -> i64 {

    if divisors.contains_key(&n) {
        return *divisors.get(&n).unwrap();
    }

    let mut cnt:i64 = 2;

    for x in (2..n/2+1) {
        if n %x == 0 {
            cnt += 1;
        }
    }

    divisors.insert(n, cnt);

    return cnt;
}

fn get_divisors_triangle(n:i64, divisors: &mut HashMap<i64, i64>) -> i64 {

    // Get divisors of nth triangle number
    // Triangle number => n(n+1)/2
    // So if n is odd the divisors
    // is divisors(n)*divisors(n+1/2)
    // If n is even, divisors is
    // divisors(n/2)*divisors(n+1)

    if n % 2 == 0 {
        return get_divisors(n/2, divisors)*get_divisors(n+1, divisors);
    }
    else {
        return get_divisors(n, divisors)*get_divisors((n+1)/2, divisors);
    }
}

fn get_triangle_number(n:i64) -> i64 {

    // A triangle number for n is nothing but some of all natural
    // numbers upto and including n

    return n*(n+1)/2;    
    
}

fn get_first_with_n_divisors(n:i64) -> i64 {

    let mut number:i64 = 100;
    let mut n_divisors = 0;
    let mut divisors: HashMap<i64, i64> = HashMap::new();
    
    while true {
        n_divisors = get_divisors_triangle(number, &mut divisors);
        if n_divisors >= n {
            break;
        }

        number += 1;
    }

    return get_triangle_number(number);
}

fn main() {

    println!("{}", get_first_with_n_divisors(500));

}



