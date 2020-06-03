// Largest pandigital prime

use std::iter::FromIterator;

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


fn largest_pandigital_prime() {
    
    let template: String = "123456789".to_string();
    // We start with 7654321 - why ?
    // Cuz 987654321 and its permuations are divisible by 9 (sum: 45)
    // Similarly 87654321 and its permutations are also divisible by 9 (sum: 36)
    let mut n: u64 = 7654321;
//    let mut primes: HashMap<u64, bool> = HashMap::new();

    // Cuz this was given in the problem as a possible lower limit
    while n > 2143 {
        let n_str: String = n.to_string();
        let mut n_str_chars: Vec<char> = n_str.chars().collect();
        n_str_chars.sort_by(|a,b| a.cmp(b));
        
        let n_template:String = template[..n_str.len()].to_string();
        // println!("{}", n_template);
        if String::from_iter(n_str_chars) == n_template {
            println!("Trying {}",n);
            
            if is_prime(n) { 
                println!("Largest => {}", n);
                break;
            }
        }

        // Cuz only odd numbers can be prime
        n -= 2;
    }
}

fn main() {
    largest_pandigital_prime();
}
