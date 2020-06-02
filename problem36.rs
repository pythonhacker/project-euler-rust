// Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.

// Convert number to base2
fn num_to_base2(n: u64) -> String {

    let mut num = n;
    let mut rem: u64;
    let mut base_two: String = "".to_string();
    
    while num > 0 {
        rem = num % 2;
        num = num / 2;

        base_two.push_str(&rem.to_string());
    }

    base_two = base_two.chars().rev().collect::<String>();
    return base_two;
}

fn is_palindrome(s: String) -> bool {

    let mut svec: Vec<String> = s.chars().map( |x| x.to_string()).collect();
//    println!("{:?}", svec);

    while !svec.is_empty()  {
        let first:String = svec.first().unwrap().to_string();
        if first != svec.pop().unwrap() {
            return false;
        }

        if svec.len() > 0 {
            svec.remove(0);
        }
    }
        
    return true;
}

fn main() {

    let mut sum:u64 = 0;
    
    for x in 1u64..1000000u64 {
        if is_palindrome(x.to_string()) && is_palindrome(num_to_base2(x)) {
            sum += x;
            println!("{}", x);
        }
    }

    println!("{}", sum);
}
