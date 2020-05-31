
fn is_prime(n: i64)  -> bool {

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

fn count_primes(a: i64, b: i64) -> i64 { 

    let mut val: i64;
    let mut n:i64 = 0;
    
    loop {
        val = n*(n+a) + b;
        if val<2 { break; }
        
        if !is_prime(val) {
            break;
        }
        n += 1;
    }
    
    return n;
}

fn quad_prime() {

    let mut max_run: i64 = 0;
    let mut max_a: i64 = 0;
    let mut max_b: i64 = 0;

    for a in -999..1000 {
        for b in -1000..1001 {

            let n: i64 = count_primes(a, b);
            if n > max_run {
                println!("Max run => {}, {}, {}", max_run, a, b);
                max_run = n;
                max_a = a;
                max_b = b;
            }
        }
    }
    
    println!("n:{} a:{} b:{}", max_run, max_a, max_b);
    println!("Product => {}", max_a*max_b);    
}

fn main() {

    quad_prime();
}

