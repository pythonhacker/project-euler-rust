// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn triplet_finder(n:i32) -> (i32, i32, i32) {

    let mut triplet = (0,0,0);
    let mut z:i32;
    
    for x in (1..n+1) {
        for y in (1..n+1) {
            if x+y>n { continue; }
            z = n - (x+y);
            if x*x + y*y == z*z  {
                triplet = (x,y,z);
                break;
            }
        }
    }
            
    return triplet;
}

fn main() {

    let t = triplet_finder(1000);
    println!("{}", t.0*t.1*t.2);
}
