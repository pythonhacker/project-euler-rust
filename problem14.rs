// Longest collatz sequence

fn collatz(n: i64) -> i64 {
    
    let (mut ncopy, mut count, mut nval) = (n,1,0);

    while ncopy != 1 {
        if ncopy % 2 == 0 {
            ncopy = ncopy/2;
        } else {
            ncopy = 3*ncopy + 1;
        }

        count += 1;
    }

    return count;
}

fn find_largest(n:i64) -> (i64, i64) {

    let (mut largest, mut cur_n, mut cn) = (0,0,0);

    for x in (1..n+1) {
        cn = collatz(x);
//        println!("Collatz {} => {}", x, cn);
        if cn > largest {
            largest = cn;
            cur_n = x;
        }
    }

    return (largest, cur_n);
}

fn main() {

    println!("{:?}", find_largest(1000000));
}
