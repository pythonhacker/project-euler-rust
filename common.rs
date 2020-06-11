
// Common functions

#[allow(dead_code)]        
pub fn digit_to_vector(num:u64) -> Vec<u64> {

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
#[allow(dead_code)]
pub fn vector_to_digit(num_vec: &Vec<u64>) -> u64 {

    let mut num: u64 = 0;
    let mut i: u64 = 0;

    for n in num_vec.iter().rev() {
        num += 10u64.pow(i as u32)*n;
        i += 1;
    }

    return num;
}

#[allow(dead_code)]
pub fn digit_to_vector_32(num:u32) -> Vec<u32> {

    let mut vdigits: Vec<u32> = vec![];
    let mut n = num;

    while n > 0 {
        vdigits.push(n%10);
        n = n / 10;
    }

    vdigits.reverse();

    return vdigits;
}

// [1,2,3,4] => 1234
#[allow(dead_code)]    
pub fn vector_to_digit_32(num_vec: Vec<&u32>) -> u32 {

    let mut num: u32 = 0;
    let mut i: u32 = 0;

    for n in num_vec.iter().rev() {
        num += 10u32.pow(i)*(*n);
        i += 1;
    }

    return num;
}


// Is n a prime ?
#[allow(dead_code)]        
pub fn is_prime(n: u64)  -> bool {

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

#[allow(dead_code)]        
pub fn is_prime_32(n: u32)  -> bool {

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

#[allow(dead_code)]        
pub fn is_prime_i64(n: i64)  -> bool {

    let mut flag = true;
    let mut item: i64 = 2;

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


#[allow(dead_code)]        
pub fn is_prime_i64_ex(number: i64, primes: &Vec<i64>) -> bool {
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

#[allow(dead_code)]        
pub fn get_prime_factors_32(n: u32) -> Vec<u32> {

    let mut p: u32 = 3;
    let mut factors: Vec<u32> = vec![];

    if n % 2 == 0 {
        factors.push(2);
    }
    
    loop {
        if n % p == 0 && is_prime_32(p) {
            factors.push(p);
        }

        if p >= n/2 { break; }
        p += 2;
    }
        
    return factors;
}


// Return a vector of primes < a given limit
#[allow(dead_code)]        
pub fn consecutive_primes(limit: u64) -> Vec<u64> {

    let mut n = 3;
    let mut primes:Vec<u64> = vec![2];

    while n<limit {
        if is_prime(n) {
            primes.push(n);
        }

        n += 2;
    }
        
    return primes;
}

#[allow(dead_code)]        
pub fn largest_prime_factor(n: u64)->u64 {

    let x = (n as f64).sqrt() as u64 + 1;
    let mut val = 0;
    
    for item in (2..x).rev() {
        if n % item == 0 && is_prime(item) {
            val = item;
            break;
        }
    }
        
    return val;

}

#[allow(dead_code)]        
pub fn gcd(mut x:u64, mut y:u64)->u64 {

    let mut temp: u64;
    
    while y > 0 {
        temp = x;
        x = y;
        y = temp % y;
    }

    return x;
}

#[allow(dead_code)]        
pub fn find_gcd(numbers: &Vec<u64>)->u64 {

    let x:u64 = numbers[0];
    let y:u64 = numbers[1];

    let num_v: Vec<u64> = numbers[2..].to_vec();

    let mut n_gcd:u64 = gcd(x, y);

    for num in num_v.iter() {
        n_gcd = gcd(n_gcd, *num);
    }

    return n_gcd;

}

#[allow(dead_code)]        
pub fn lcm(x:u64, y:u64) -> u64 {

    let (num, den, n_gcd, n_lcm);
    
    if x>y {
        num = x;
        den = y;
    } else {
        num = y;
        den = x;
    }

    n_gcd = gcd(den, num);

    n_lcm = x*y/n_gcd;

    return n_lcm;
}

#[allow(dead_code)]        
pub fn find_lcm(numbers: &Vec<u64>) -> u64 {


    let x:u64 = numbers[0];
    let y:u64 = numbers[1];

    let num_v: Vec<u64> = numbers[2..].to_vec();

    let mut n_lcm:u64 = lcm(x, y);


    for num in num_v.iter() {
        n_lcm = lcm(n_lcm, *num);
    }

    return n_lcm;    

}

#[allow(dead_code)]        
pub fn gcd_32(mut x: u32, mut y: u32)->u32 {

    let mut temp: u32;
    
    while y > 0 {
        temp = x;
        x = y;
        y = temp % y;
    }

    return x;
}

