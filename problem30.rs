// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

// Sum of numbers whose digits to given power equals the number itself
fn sum_integer_powers(power: u32) {

    let mut number: u64;
    let mut num: u64;
    let mut num_pow: u64;
    let limit: u64;
    let mut sum: u64;
    
    number = 10u64.pow(power-2);
    limit = 10u64.pow(power+1) - 1;
    sum = 0;
    
    loop {
        num_pow = 0;
        num = number;
        
        while num > 0 {
            num_pow += (num % 10).pow(power);
            num = num / 10;
        }

        if number == num_pow {
            println!("{}", number);
            sum += number;
        }

        number += 1;
        if number == limit { break; }
    }    

    println!("Sum => {}", sum);
}

fn main() {
    sum_integer_powers(5);
}
