// Distinct prime factors

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

fn get_prime_factors(n: u32) -> Vec<u32> {

    let mut p: u32 = 3;
    let mut factors: Vec<u32> = vec![];

    if n % 2 == 0 {
        factors.push(2);
    }
    
    loop {
        if n % p == 0 && is_prime(p) {
            factors.push(p);
        }

        if p >= n/2 { break; }
        p += 2;
    }
        
    return factors;
}

fn consecutive_prime_factors(min_p: usize) -> Vec<u32> {

    let mut n_range = 0..1;
    
    if min_p == 2 {
        n_range = 10..100;
    } else if min_p == 3 {
        n_range = 100..1000;
    } else if min_p == 4 {
        n_range = 10000..1000000;
    }

    let mut idx: usize = 0;
    let mut numbers: Vec<u32> = vec![];
    
    for n in n_range {
        let mut p_factors = get_prime_factors(n);
        if p_factors.len() >= min_p {
            numbers.push(n);

            if (idx>0) && (n - numbers[idx-1]) > 1 {
                numbers.clear();
                idx = 0;
            } else {
                idx += 1;
                println!("{:?}", numbers);
            }
        } 

        if idx == min_p {
            break;
        }
    }
    
    return numbers;
}

fn main () {
    let _factors = consecutive_prime_factors(4);
    println!("{:?}", _factors);
    for n in _factors.iter() {
        println!("Prime factors of {} => {:?}", *n, get_prime_factors(*n));
    }
}

