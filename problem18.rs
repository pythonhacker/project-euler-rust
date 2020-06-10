// NOTE: Problem18 is unsolved!
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

struct TriangleGrid {
    data: Vec<Vec<u32>>,
    height: usize
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl TriangleGrid {

    fn read(&mut self, filename: String) {

        if let Ok(lines) = read_lines(filename) {
            let mut i = 0;
            
            for line in lines {
                if let Ok(numbers) = line {
                    let v: Vec<u32> = numbers.split(" ").map( |d| u32::from_str(d).unwrap()).collect();
//                    println!("{:?}", v);
                    self.data[i] = v;
                    i += 1;
                }
            }

            self.height = i;
        }
    }

    fn max_sum(&self)->u32 {

        let mut max_s: u32 = 0;
        let mut curr_sum: u32;
        let mut last_sum: u32 = 0;        
        let mut last_idx: usize = 0;
        
        for i in 0..self.height {
            let row = &self.data[i];
            
            if i == 0 {
                last_idx = 0;
                max_s = row[0];
                last_sum = row[0];
            } else {
                let mut j_last: usize = 0;
                
                for j in 0..row.len() {
                    let mut diff;
                    if j >= last_idx {
                        diff = j - last_idx;
                    } else {
                        continue;
                    }
                    
                    if diff <= 1  {
                        println!("Trying index {}",j);
                        curr_sum = last_sum + row[j];
                        if curr_sum > max_s {
                            max_s = curr_sum;
                            j_last = j;
                        }
                    }
                }

                last_sum = max_s;
                last_idx = j_last;
                println!("row: {}, num: {}", i+1, row[last_idx]);
                
            }
            
            println!("{:?}",row);
        }

        return max_s;
    }
}

fn main() {
    let mut mytriangle = TriangleGrid { data: vec![vec![0; 50]; 50], height: 0 };
    mytriangle.read("triangle.txt".to_string());
    println!("{}", mytriangle.max_sum());

}
