
extern crate num_bigint;
// extern crate num_traits;

use num_bigint::{BigUint};

fn fibonacci_big() {

    let mut a: BigUint = 0u32.into();
    let mut b: BigUint = 1u32.into();
    let mut c: BigUint = 0u32.into();
    let mut index: i32 = 1;
    
    loop {
        c = a + b.to_owned(); // Rust ownership rules bites us here
        a = b;
        b = c.to_owned(); // Here too.
        index += 1;
        
        if c.to_string().len() == 1000 {
            println!("Index: {}, Number: {}", index, c);
            break;
        }
    }
    
}

fn main() {
    fibonacci_big();
}
