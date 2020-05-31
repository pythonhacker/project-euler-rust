// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
// This implementation uses the suggestion at https://projecteuler.net/quote_post=13-f853fd887b0a6b53247dfb4e61ab628785585290

fn max_recurrence(limit: u64) {

    let mut length: u32;
    let mut max_length: u32 = 0;
    let mut max_n: u64 = 0;
    
    for n in 2..limit+1 {
        let mut rest: u64 = 1;
        let mut r0: u64;

        for _ in 0..n  {
            rest = (rest*10) % n;
        }
        r0 = rest;
        length = 0;

        loop {
            rest = (rest*10) % n;
            length += 1;
            if rest == r0 {
                break;
            }
        }

        if length > max_length {
            max_length = length;
            max_n = n;
        }
    }

    println!("{} {}", max_n, max_length);
}

fn main() {
    max_recurrence(1000);
}
