// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

use std::collections::HashMap;

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

fn is_abundant(n: i64, divisors: &mut HashMap<i64, i64>, cache: &mut HashMap<i64, bool>) -> bool {

    // Smallest abundant number is 12
    if n < 12 {
        return false;
    }

    // Use a cache
    if cache.contains_key(&n) {
        return *cache.get(&n).unwrap();
    }
    
    let sum_d = get_divisor_sum(n, divisors);

    if sum_d > n {
        // Abundant number
        cache.insert(n, true);        
        return true;
    } else {
        // Deficient or perfect number
        cache.insert(n, false);        
        return false;
    }
}

fn non_abundant_sum(limit: i64) -> i64 {

    let mut k: i64;
    let mut flag: bool;
    let mut sum: i64 = 0;
    let mut divisors: HashMap<i64, i64> = HashMap::new();
    let mut cache: HashMap<i64, bool> = HashMap::new();    
    
    for i in 1..limit+1 {
        flag = false;

        for j in 1..(i/2+1) {
            k = i - j;
            if is_abundant(j, &mut divisors, &mut cache) {
                if is_abundant(k, &mut divisors, &mut cache) {
//                println!("Abundant sum of {} is {} + {}", i, j, k); 
                    flag = true;
                    break;
                }
            }
        }

        if !flag {
            sum += i;
        }
    }

    return sum;
}

fn main() {
//    println!("{}", non_abundant_sum(100));
    println!("{}", non_abundant_sum(28123));    
}
