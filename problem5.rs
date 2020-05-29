// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn gcd(mut x:i64, mut y:i64)->i64 {

    let mut temp: i64;
    
    while y > 0 {
        temp = x;
        x = y;
        y = temp % y;
    }

    return x;
}

fn find_gcd(numbers: &Vec<i64>)->i64 {

    let x:i64 = numbers[0];
    let y:i64 = numbers[1];

    let num_v: Vec<i64> = numbers[2..].to_vec();

    let mut n_gcd:i64 = gcd(x, y);

    for num in num_v.iter() {
        n_gcd = gcd(n_gcd, *num);
    }

    return n_gcd;

}

fn lcm(x:i64, y:i64) -> i64 {

    let (mut num, mut den, mut n_gcd, mut n_lcm)=(0,0,0,0);
    
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

fn find_lcm(numbers: &Vec<i64>) -> i64 {


    let x:i64 = numbers[0];
    let y:i64 = numbers[1];

    let num_v: Vec<i64> = numbers[2..].to_vec();

    let mut n_lcm:i64 = lcm(x, y);


    for num in num_v.iter() {
        n_lcm = lcm(n_lcm, *num);
    }

    return n_lcm;    

}

fn main() {

    let numbers: Vec<i64>  = (2..21).collect();
    println!("{:?}", &numbers);
    
    println!("{}", find_gcd(&numbers));
    println!("{}", find_lcm(&numbers));

}
