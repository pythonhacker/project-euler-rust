// Find the sum of the only eleven primes that are both truncatable from left to right and right to left.

// Is n a prime ?
fn is_prime(n: u64)  -> bool {

    if n <= 1 {
        return false;
    }
            
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

fn is_truncatable_prime(n: u64) -> bool {

    let n_vec = digit_to_vector(n);
    let mut n_slice_num: u64;
    
    for i in 1..n_vec.len() {
        let mut n_slice_left = &n_vec[i..];
        n_slice_num = vector_to_digit(&n_slice_left.to_vec());
        
        // println!("{}", n_slice_num);        
        if !is_prime(n_slice_num) {
            return false;
        }
    }

    for i in (1..n_vec.len()).rev() {
        let mut n_slice_right = &n_vec[..i];
        n_slice_num = vector_to_digit(&n_slice_right.to_vec());

        // println!("{}", n_slice_num);                
        if !is_prime(n_slice_num) {
            return false;
        }
    }    

    return true;
}

fn main() {

    let mut sum: u64 = 0;
    
    for x in 11..10000000 {
        if !is_prime(x) {continue;}
        if is_truncatable_prime(x) {
            println!("{}", x);
            sum += x;
        }
    }

    println!("Sum => {}", sum);
}
