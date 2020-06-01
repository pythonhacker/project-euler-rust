// Find the sum of all numbers which are equal to the sum of the factorial of their digits.

fn factorial(n: u64) -> u64 {
    return (2..n+1).fold(1u64, |p,x| p*x);
}

// Is number curious ?
fn curious(n: u64) -> bool {

    let mut num = n;
    let mut sum: u64 = 0;
    
    while num > 0 {
        sum += factorial(num%10);
        num = num/10;
    }    

    return sum == n;
}

fn main() {

    let mut sum: u64 = 0;
    
    for i in 11u64..1000000u64 {
        // No point in doing factorials which are multiples of 10
        if i % 10 == 0 { continue; }
        if curious (i) {
            println!("{}", i);
            sum += i;
        }
    }

    println!("{}",sum);

}
    
