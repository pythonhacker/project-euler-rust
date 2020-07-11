// Pandigital products

extern crate itertools;

use itertools::Itertools;
use std::iter::FromIterator;

mod common;

pub use common::{vectorp_to_digit_u32};

// Return sum of pandigital product using two multiplicands axb
// where #digits(a) -> r1 and #digits(b) -> r2
fn pandigital_product(r1: usize, r2: usize) -> u32{

    let numbers: Vec<u32> = vec!(1,2,3,4,5,6,7,8,9);
    let template: String = "123456789".to_string();
    let mut products: Vec<u32> = vec![];
    
    let mut sum: u32 = 0;
    
    for p1 in numbers.iter().permutations(r1) {
        let mut n2: Vec<u32> = vec![];

        for i in numbers.iter() {
            if p1.iter().position( |r| *r == i) == None {
                n2.push(*i);
            }
        }

        let d1: u32 = vectorp_to_digit_u32(p1);
        let mut d2: u32;
        let mut d3: u32;
        let mut prod_str: String;
        let mut prod_str_chars: Vec<char>;
        
        for p2 in n2.iter().permutations(r2) {
            d2 = vectorp_to_digit_u32(p2);
            d3 = d1*d2;
            prod_str = format!("{}{}{}",d1,d2,d3);
            // println!("{}", prod_str);
            prod_str_chars = prod_str.chars().collect();
            prod_str_chars.sort_by(|a,b| a.cmp(b));

            if String::from_iter(prod_str_chars) == template {
                println!("{} {} {}",d1,d2,d3);
                if products.iter().position( |r| *r == d3) == None {
                    sum += d3;
                    products.push(d3);
                }
            }

        }
    }

    return sum;
}

fn main() {
    let mut sum: u32 = pandigital_product(2,3);
    sum += pandigital_product(1, 4);

    println!("{}", sum);
}
