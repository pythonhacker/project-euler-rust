use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate num_bigint;

use num_bigint::{BigUint};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

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
