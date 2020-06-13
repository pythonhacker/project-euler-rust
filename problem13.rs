extern crate num_bigint;
use num_bigint::{BigUint};

mod common;
use common::{read_lines};

fn sum_numbers(filename: &str) {

    let mut sum: BigUint = 0u32.into();
    
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(number) = line {
                let nb = number.as_bytes();
                // println!("{:?}", nb);
                let num: BigUint = BigUint::parse_bytes(nb, 10).unwrap();
                // println!("{}", num);
                sum += num;
            }
        }
    }

    println!("{}", &sum.to_string()[0..10]);        
    
}

fn main() {
    sum_numbers("numbers.txt");
}
