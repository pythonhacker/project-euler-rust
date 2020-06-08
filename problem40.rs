// An irrational decimal fraction is created by concatenating the positive integers:
// 0.123456789101112131415161718192021...

// It can be seen that the 12th digit of the fractional part is 1.
// If dn represents the nth digit of the fractional part, find the value of the following expression.

// d1*d10*d100*d1000*d10000*d100000*d1000000

fn champernowne_product() {

    let mut n: u64 = 1;
    let mut prod: u64 = 1;
    let mut offset: u64 = 0;

    let offsets: Vec<u64> = vec!(1,10,100,1000,10000,100000, 1000000);
    
    loop {

        for c in n.to_string().chars() {
            offset += 1;
            if offsets.iter().position( |r| *r == offset) != None {
                println!("Digit at offset {} is {}",offset,c);
                prod *= c.to_digit(10u32).unwrap() as u64;
            }
        }

        n += 1;
        if offset >= 1000000 {
            break;
        }
        
    }

    println!("{}", prod);
}

fn main() {
    champernowne_product();
}
        
