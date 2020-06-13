// Digit canceling fractions

mod common;
use common::{digit_to_vector_u32, gcd_u32};

fn main() {

    let mut f1: f32;
    let mut f2: f32 = 0.0;
    let mut prod_num: u32 = 1;
    let mut prod_denom: u32 = 1;
    
    for n in 11..98 {
        // Avoid multiples of 10
        if n % 10 == 0 { continue; }
        
        let mut nvec = digit_to_vector_u32(n);
        // Avoid trivial numbers like 11, 22, 33 etc
        if nvec[0] == nvec[1] { continue; }
        
        for d in n+1..100 {
            if d % 10 == 0 { continue; }
            
            let mut dvec = digit_to_vector_u32(d);
            if dvec[0] == dvec[1] { continue; }            

            f1 = (n as f32)/(d as f32);
            if nvec[1] == dvec[0] {
                f2 = (nvec[0] as f32)/(dvec[1] as f32);
            } else if nvec[0] == dvec[1] {
                f2 = (nvec[1] as f32)/(dvec[0] as f32);
            }
            
            if f1 == f2 {
                println!("{} {}", n, d);
                prod_num *= n;
                prod_denom *= d;
            }
        }
    }

    // Get highest common factor
    let factor = gcd_u32(prod_num, prod_denom);
    // Divide it by the denominator - that is the answer.
    println!("{}", prod_denom/factor);

}
