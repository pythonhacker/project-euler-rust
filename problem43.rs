// Sub-string divisibility
extern crate itertools;

use itertools::Itertools;

fn check_sub_divisibility(n_vec: &Vec<u64>) -> bool {    

    // Already taking care of 2,3 and 5 divisibility in main loop
    let d4: bool = vector_to_digit(&n_vec[4..7].to_vec()) % 7 == 0;
    let d5: bool = vector_to_digit(&n_vec[5..8].to_vec()) % 11 == 0;
    let d6: bool = vector_to_digit(&n_vec[6..9].to_vec()) % 13 == 0;
    let d7: bool = vector_to_digit(&n_vec[7..].to_vec()) % 17 == 0;

    return [d4,d5,d6,d7].iter().fold(true, |p,x| p &x);
}

fn vector_to_digit(num_vec: &Vec<u64>) -> u64 {

    let mut num: u64 = 0;
    let mut i: u64 = 0;

    for n in num_vec.iter().rev() {
        num += 10u64.pow(i as u32)*n;
        i += 1;
    }

    return num;
}

fn pandigital_divisible_sum() -> u64 {
    
    let mut sum: u64 = 0;

    for it in (0..10).permutations(10) {
        // println!("{:?}", it);
        if it[0] == 0 { continue; }

        // 5th index has to be 5 or 0 to be divisible by 5
        if it[5] != 0 && it[5] != 5 {
            continue;
        }        

        // Number at 3rd index has to be divisible by 2
        if it[3] % 2 != 0 {
            continue;
        }

        // Sum of [2..5] index to be divisible by 3
        if (it[2] + it[3] + it[4]) % 3 != 0 {
            continue;
        }
        
        if check_sub_divisibility(&it) {
            let n = vector_to_digit(&it);
            println!("{}", n);
            sum += n;
        }
    }

    return sum;
}

fn main() {

    println!("Sum -> {}", pandigital_divisible_sum());
}
