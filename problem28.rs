// Sum of spiral diagonal

// Analysis: You will find each member of the spiral diagonal at level
// n is given as (n-2**2 + n-1 .. n**2 + 1).step_by(n-1)

fn sum_spiral_diagonal(n: u64)-> u64 {

    if n == 1 { return 1; }
    
    let start: u64 = (n-2).pow(2) + n -1;
    let end: u64 = n.pow(2) + 1;
    
    let sum = (start..end).step_by( (n as usize)-1).fold(0, |s, x| s+x);
    return sum;
}

fn main() {

    let sum = (1..1002).step_by(2).map(|n| sum_spiral_diagonal(n as u64)).fold(0, |s,x| s+x);
    println!("{}",sum);
}
