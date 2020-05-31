// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

// Uses the technique described in https://projecteuler.net/quote_post=361-80066e82e1815a5440e0758e0cf2c1ee3aa0276f

fn reduce_index(n: u64) -> u64 {

    let mut quotient: u64;
    let mut num = n;
    let mut fact: u64;
    let mut num_idx: String = "".to_string();
    
    let mut digits: String = "0123456789".to_string();
    let mut permute_string: String = "".to_string();
    
    for i in (0..10).rev() {
        // inline factorial
        fact = (2..i+1).fold(1, |p, x| p*x);
        quotient = num/fact;

        let mut idx = 0;

        // num_idx => number in digits at idx where idx == quotient
        for c in digits.chars() {
            if (idx as u64) == quotient {
                num_idx = c.to_string();
                break;
            }
            idx += 1;
        }
        num = num % fact;
        permute_string.push_str(&num_idx);
        // Drop num_idx from digits
        digits = digits.replace(&num_idx, "");
    }

    return permute_string.parse::<u64>().unwrap();
}

fn main() {
    println!("{}", reduce_index(999999));
}
