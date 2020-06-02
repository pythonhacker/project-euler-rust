// If p is the perimeter of a right angle triangle with integral length sides, {a,b,c},
// there are exactly three solutions for p = 120.

// {20,48,52}, {24,45,51}, {30,40,50}

// For which value of p \u2264 1000, is the number of solutions maximised?

fn triplet_finder(p: u64) -> Vec<(u64,u64,u64)> {

    let mut triplets = vec![];
    let p_root: u64 = ((p as f64).sqrt() as u64) + 1;
    let mut z:u64;
    
    for x in p_root..p {
        for y in p_root..p {
            if x+y > p {continue;}

            z = p - (x+y);
            if z<=x || z <=y {
                continue;
            }

            if x*x + y*y == z*z {
                let mut sides:Vec<u64> = vec![x,y,z];
                sides.sort();
                
                let mut t = (sides[0], sides[1], sides[2]);
                if triplets.iter().position( |r| *r == t) == None {
                    triplets.push(t);
                }
            }
        }
    }
                    

    return triplets;
}

fn main() {

    let mut l_max: usize = 0;
    let mut n_max: u64 = 0;
    
    for x in 10u64..1000u64 {
        let mut triplets = triplet_finder(x);
        if triplets.len() > l_max {
            l_max = triplets.len();
            n_max = x;
            println!("{} {}", l_max, n_max);
        }
    }
    
    println!("Max => {} {}", n_max, l_max);
}
