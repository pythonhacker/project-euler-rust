// Sum of amicable numbers under 10000.

use std::collections::HashMap;


// Return sum of divisors of a given number
fn get_divisor_sum(n: i64, divisors: &mut HashMap<i64, i64>) -> i64 {

    if divisors.contains_key(&n) {
        return *divisors.get(&n).unwrap();
    }

    let mut sum: i64 = 1;

    for x in 2..(n/2 + 1) {
        if n % x == 0 {
            sum += x;
        }
    }

    divisors.insert(n, sum);
    
    return sum;
}


fn amicable(n: i64, amicables: &mut Vec<i64>) -> i64 {

    let mut divisors: HashMap<i64, i64> = HashMap::new();
    // Get divisor sum of n
    let s1: i64 = get_divisor_sum(n, &mut divisors);
    let s2: i64;
    
    if s1 == n {
        // a == b, cannot be an amicable pair
        return 0;
    }

    // Get divisor sume of s1 (which is divisor sum of n)
    s2 = get_divisor_sum(s1, &mut divisors);

    // d(a) = b && d(b) = a
    if n == s2 {
        //        println!("{} {}", n, s1);

        // Is an amicable pair
        amicables.push(n);
        amicables.push(s1);

        // Return their sum
        return n + s1;
    } else {
        // Not amicable
        return 0;
    }
}

fn main() {

    let mut sum: i64 = 0;
    let mut amicables: Vec<i64> = vec![];

    for x in 2..10000 {
        if amicables.contains(&x) {
            continue;
        }

        sum += amicable(x, &mut amicables);
    }

    println!("{}", sum);
}
