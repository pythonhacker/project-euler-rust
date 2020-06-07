// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn sum_squares(n:i64)->i64 {

    let mut s:i64 = 0;

    for i in 1..n+1 {
        s += i*i;
    }

    return s;
}

fn square_sum(n:i64)->i64 {
    let mut s:i64 = 0;

    for i in 1..n+1 {
        s += i;
    }

    return s*s;
}

fn main() {

    let diff: i64 = square_sum(100) - sum_squares(100);
    println!("{}", diff);
    
}
