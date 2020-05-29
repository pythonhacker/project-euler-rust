// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is
// 9009 = 91 x 99.
// Find the largest palindrome made from the product of two 3-digit numbers.


fn is_palindrome(number: i64) -> bool {

    let mut n = number;
    let mut rev = 0;
    let mut dig;
    
    while n > 0 {
        dig = n % 10;
        rev = rev * 10 + dig;
        n = n / 10;
    }

    return number == rev;
}

fn largest_palindrome(number: i32, diff: i32) -> i32 {

    let (n1,n2) = (number-1, number-1);
    let (mut largest, mut n) = (0, 0);
    let (mut max_x, mut max_y) = (0, 0);

    for x in (n1-diff..n1).rev() {
        for y in (n2-diff..n2).rev() {
            n = x*y;
            if is_palindrome(n as i64) && n>largest {
                largest = n;
                max_x = x;
                max_y = y;
            }
        }
    }

    return largest;
}

fn main() {
    println!("{}",is_palindrome(100002));
    println!("{}", largest_palindrome(1000,100));
}
