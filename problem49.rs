extern crate itertools;

use itertools::Itertools;

// [1,2,3,4] => 1234
fn vector_to_digit(num_vec: Vec<&u32>) -> u32 {

    let mut num: u32 = 0;
    let mut i: u32 = 0;

    for n in num_vec.iter().rev() {
        num += 10u32.pow(i)*(*n);
        i += 1;
    }

    return num;
}

fn digit_to_vector(num:u32) -> Vec<u32> {

    let mut vdigits: Vec<u32> = vec![];
    let mut n = num;

    while n > 0 {
        vdigits.push(n%10);
        n = n / 10;
    }

    vdigits.reverse();

    return vdigits;
}

fn is_prime(n: u32)  -> bool {

    let mut flag = true;
    let mut item: u32 = 2;

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

fn prime_permute() -> String {

    let mut n:u32 = 1001;
    let mut count: usize = 0;
    
    loop {
        if is_prime(n) {
            let mut n_vec: Vec<u32> = digit_to_vector(n);
            let mut n_prime: Vec<u32> = vec![n];
            let mut idx: usize = 0;
            
            for it in n_vec.iter().permutations(4) {
                if *it[0] == 0 { continue; }

                let mut n_perm: u32 = vector_to_digit(it.to_vec());
                
                if is_prime(n_perm) && (n_perm > n_prime[idx]) && ((n_perm - n_prime[idx]) == 3330) {
                    n_prime.push(n_perm);
                    idx += 1;
                }
            }

            if n_prime.len() == 3 {
                count += 1;
                println!("{:?}", n_prime);

                if count == 2 {
                    return n_prime.iter().map( |x| x.to_string() ).join("");
                }
            }
        }

        n += 1;
        if n == 9999 { break; }
    }

    return "".to_string();
}

fn main() {
    println!("{}", prime_permute());
}
