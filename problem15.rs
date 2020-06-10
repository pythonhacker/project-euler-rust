//  Lattice paths
use std::env;

// Recursive solution
fn npaths_r(x: i64, y: i64, h: i64, w: i64)->i64 {

    if x == w || y == h {
        return 1;
    }
    else {
        return npaths_r(x+1, y, h, w) + npaths_r(x, y+1, h, w);
    }
}

// Iterative solution
fn npaths_i(n: u128) -> u128 {

    //Answer: 2nCn -> (2n)!/(n!*n!)
    let twicen: u128 = 2*n;
    let mut prod1: u128 = 1;
    let mut prod2: u128 = 1;

    // Why this round-about ? Because in Rust apparently the largest
    // integer is 2**128. factorial(40) is way above that!
    // This is (n+1)*(n+2)...2n or (2n!)/(n!)
    for x in n+1..twicen+1 {
        prod1 *= x;
    }

    // This is n!
    for x in 1..n+1 {
        prod2 *= x;
    }    

    // This is (2n)!/(n!*n!)
    return prod1/prod2;
}

fn main() {
    // Uncomment below line for recursive solution
    // Warning: Its very computational intensive and takes a while to run.

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "-r" {
        println!("Using recursion");
        println!("{}", npaths_r(0,0,20,20));
    } else {
        println!("{}", npaths_i(20));
    }
}
