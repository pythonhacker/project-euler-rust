use std::iter::FromIterator;

fn largest_pandigital() -> u64 {
    
    let template: String = "123456789".to_string();
    let mut n: u64 = 2;
    let mut max_n: u64;
    let mut max_sofar: u64 = 1;
    
    while n < 1000000 {
        let mut j:u64 = 1;
        let mut count: usize = 0;
        let mut np: Vec<String> = vec![];
        let mut prod: String;
        
        loop {
            prod = (n*j).to_string();
            count += prod.len();

            np.push(prod);

            j += 1;
            if count >= 9 {
                break;
            }
        }

        // Convert Vec<String> to a single string
        let pan_str: String = np.join("");

        // Sort this string lexicographically
        let mut pan_str_chars: Vec<char> = pan_str.chars().collect();
        pan_str_chars.sort_by(|a,b| a.cmp(b));

        // Compare with template
        if String::from_iter(pan_str_chars) != template {
            n += 1;
            continue;
        }

        let pan_now: u64 = pan_str.parse::<u64>().unwrap();
        
        if pan_now > max_sofar {
            max_sofar = pan_now;
            max_n = n;
            println!("{} {}", max_sofar, max_n);
        }

        n += 1;
    }

    return max_sofar;
}

fn main() {

    println!("Largest => {}", largest_pandigital());
}
