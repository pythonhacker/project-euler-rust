// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

fn is_prime(n: i64) -> bool {

    let x = (n as f64).sqrt() as i64 + 1;
    let mut flag = true;
    
    for item in 2..x {
        if n % item == 0 {
            flag = false;
            break;
        }
    }

    return flag;
}

fn largest_prime_factor(n: i64)->i64 {

    let x = (n as f64).sqrt() as i64 + 1;
    let mut val = 0;
    
    for item in (2..x).rev() {
        if n % item == 0 && is_prime(item) {
            val = item;
            break;
        }
    }
        
    return val;

}

fn main() {

    println!("{}", largest_prime_factor(600851475143));

}
