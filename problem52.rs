// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.
use std::iter::FromIterator;

fn num_to_template(n: u64) -> String {

    let n_s:String = n.to_string();
    let mut n_chars: Vec<char> = n_s.chars().collect();
    n_chars.sort_by(|a,b| a.cmp(b));

    return String::from_iter(n_chars);
}

fn same_digits_multiples(limit: u64) ->u64 {

    let mut n:u64 = 1;
    
    loop {
        let n_muls:Vec<u64> = vec!(2*n,3*n,4*n,5*n,6*n);
        let n_strings: Vec<String> = n_muls.iter().map(|s| num_to_template(*s)).collect();
        // println!("{:?}", n_strings);
        let n_s: String = n_strings[0].to_owned();
        let mut flag: bool = true;
        
        for s in n_strings.iter() {
            if *s != n_s {
                flag = false;
                break;
            }
        }

        if flag { println!("{} {:?}", n, n_muls); break; }

        n += 1;        
        if n == limit { break; }
    }

    return n;
}

fn main() {

    same_digits_multiples(1000000);

}
