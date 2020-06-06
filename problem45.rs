// Triangular, Pentagonal and Hexagonal

// Given a pentagonal number find its n
fn get_n_penta(p: u64) -> f64 {
    // Formula: P = (3n*n - n)/2
    // This gives the quadratic equation
    // 3n*n - n -2*P = 0
    // The solution for this is,
    // (1 + sqrt(1 + 24P))/6
    return (1.0 + ((24*p + 1) as f64).sqrt())/6.0
}

fn get_n_hexa(h: u64) -> f64 {
    // Formula: P = 2n*n - n
    // This gives the quadratic equation
    // 2n*n - n - P = 0
    // The solution for this is,
    // (1 + sqrt(1 + 8P))/4

    return (1.0 + ((8*h + 1) as f64).sqrt())/4.0
}

fn main() {

    let mut n: u64 = 286;
    let mut t: u64;
    let mut n1: f64;
    let mut n2: f64;
    
    loop {
        t = n*(n+1)/2;
        n1 = get_n_penta(t);
        n2 = get_n_hexa(t);

        // When we get an integer solution for this
        // that is the answer we want. This is the
        // fastest approach.
        if n1.trunc() == n1 && n2.trunc() == n2 {
            println!("{}",t);
            break;
        }

        n += 1;
    }

}
