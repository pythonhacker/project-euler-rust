// How many circular primes are there below one million?

// 1234 => [1,2,3,4]
fn digit_to_vector(num:u64) -> Vec<u64> {

    let mut vdigits: Vec<u64> = vec![];
    let mut n = num;

    while n > 0 {
        vdigits.push(n%10);
        n = n / 10;
    }

    vdigits.reverse();
    
    return vdigits;
}

// [1,2,3,4] => 1234
fn vector_to_digit(num_vec: &Vec<u64>) -> u64 {

    let mut num: u64 = 0;
    let mut i: u64 = 0;

    for n in num_vec.iter().rev() {
        num += 10u64.pow(i as u32)*n;
        i += 1;
    }

    return num;
}

// Is n a prime ?
fn is_prime(n: u64)  -> bool {

    let mut flag = true;
    let mut item: u64 = 2;

    loop {
        if item*item > n { break; }
        if n % item == 0 {
            flag = false;
            break;
        }
        item += 1;
    }
        
    return flag;
}

// All circular permutations of a number
// 123 -> [123, 231, 321].
fn circles(n: u64) -> Vec<u64> {

    let mut num_vec: Vec<u64> = digit_to_vector(n);
    let mut rotations: Vec<u64> = vec![];
    let mut i: usize = 0;
    
    // Keep rotating one to left
    loop {
        rotations.push(vector_to_digit(&num_vec));
        num_vec.rotate_left(1);
        i += 1;

        if i == num_vec.len() { break; }
    }
    
    return rotations;
}

fn main() {

    let mut count = 0;
    
    for i in 2u64..1000000u64 {
        // Do evaluation in one go using map and fold
        let all_prime: bool = circles(i).iter().map(|d| is_prime(*d)).fold(true, |p, x| p & x);
        if all_prime {
            println!("{}", i);
            count += 1;
        }
    }

    println!("Count => {}", count);

}
