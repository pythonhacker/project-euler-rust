// Maximum triangle path sum
// Brute force approach - tries all paths using recursion.

use std::str::FromStr;
use std::cmp::max;

mod common;
use common::{read_lines};

struct TriangleGrid {
    data: Vec<Vec<u32>>,
    height: usize,
    count: u32,
    max_routes: u32
}

impl TriangleGrid {

    fn read(&mut self, filename: String) {

        if let Ok(lines) = read_lines(filename) {
            let mut i = 0;
            
            for line in lines {
                if let Ok(numbers) = line {
                    let v: Vec<u32> = numbers.split(" ").map( |d| u32::from_str(d).unwrap()).collect();
                    // println!("{:?}", v);
                    self.data[i] = v;
                    i += 1;
                }
            }

            self.height = i;
            // Max routes = factorial(self.height)
            self.max_routes = 2u32.pow((self.height-1) as u32);
            println!("Max routes => {}",self.max_routes);
        }
    }

    // Return maximum at a point [x,y] in the grid where
    // x: row, and y: column
    fn max_sum(&mut self, x: usize, y: usize) -> u32 {

        if x  == self.height {
            self.count += 1;
            return self.data[x-1][y];
        }
        else {
            let row: Vec<u32> = self.data[x-1].to_owned();            
            let val1:u32 = self.max_sum(x+1,y);
            let val2:u32 = self.max_sum(x+1, y+1);
            
            return max(row[y] + val1, row[y] + val2);
        }
        
    }
}

fn main() {
    let mut mytriangle = TriangleGrid { data: vec![vec![0; 50]; 50], height: 0, max_routes: 0, count: 0};
    mytriangle.read("triangle.txt".to_string());

    println!("{}", mytriangle.max_sum(1, 0));
}
