// Pentagonal numbers

use std::collections::HashMap;

fn generate_pentagonal_numbers(limit: u128) -> Vec<u128> {

    let mut numbers: Vec<u128> = vec![];
    let mut num: u128;
    let mut n: u128 = 1;
    
    loop {
        num = n*(3*n-1)/2;
        numbers.push(num);

        n += 1;
        if n >= limit { break; }
    }

    return numbers;
}

fn pentagon_min_diffsum(numbers: Vec<u128>) -> (u128, u128, u128) {

    let mut min_diff: u128 = 2u128.pow(127);
    let mut min_p1: u128 = 0;
    let mut min_p2: u128 = 0;

    // Put the numbers in a HashMap for quick check
    let mut numbers_hash: HashMap<u128, bool> = HashMap::new();
    for p in numbers.iter() {
        numbers_hash.insert(*p, true);
    }

    // Trick is to iterate outer loop in increasing direction
    for p1 in numbers.iter() {
        // And inner loop in decreasing direction
        for p2 in numbers.iter().rev() {
            if *p1 == *p2 {continue;}

            let mut p_diff;
            if *p1 > *p2 {
                p_diff = *p1 - *p2;
            } else {
                p_diff = *p2 - *p1;
            }

            // Useful to shave off time after a first solution is found
            if p_diff > min_diff { break; }

            if !numbers_hash.contains_key(&p_diff) {
                continue;
            }            
                        
            let mut p_sum:  u128 = *p1 + *p2;
            if !numbers_hash.contains_key(&p_sum) {
                continue;
            }

            if p_diff < min_diff {
                println!("{} {} {}", p_diff, p1, p2);
                min_diff = p_diff;
                min_p1 = *p1;
                min_p2 = *p2;
            }
        }
    }

    return (min_p1, min_p2, min_diff);
}

fn main() {

    let numbers: Vec<u128> = generate_pentagonal_numbers(10000);
    println!("{:?}", pentagon_min_diffsum(numbers));

}
