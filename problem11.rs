// Maximum product in a Grid.

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

struct Grid {
    data: Vec<Vec<u32>>,
    width: usize,
    height: usize
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl Grid {

    fn read(&mut self, filename: String) {

        if let Ok(lines) = read_lines(filename) {
            let mut i = 0;
            
            for line in lines {
                if let Ok(numbers) = line {
                    let v: Vec<u32> = numbers.split(" ").map( |d| u32::from_str(d).unwrap()).collect();
                    println!("{:?}", v);
                    self.data[i] = v;
                    i += 1;
                }
            }

            // println!("{:?}", self.data);
        }
    }

    fn get_product(&self, x:usize, y:usize, orientation: &str) -> u32 {

        let (mut x2, mut y2, mut x3, mut y3, mut x4, mut y4) = (0,0,0,0,0,0);
        
        match orientation {
            "right" => {x2 = x + 1; y2 = y; x3 = x + 2; y3 = y; x4 = x + 3; y4 = y; }, 
            "left" => {x2 = x -1; y2 = y; x3 = x - 2; y3 = y; x4 = x - 3; y4 = y; },
            "bottom" => {x2=x; y2=y+1; x3 = x; y3 = y + 2; x4 = x; y4 = y + 3;}
            "diagonal-right-bottom" => {x2 = x + 1; y2 = y+1; x3 = x + 2; y3 = y+2; x4 = x + 3; y4 = y+3; }, 
            "diagonal-right-top" => {x2 = x + 1; y2 = y-1; x3 = x + 2; y3 = y-2; x4 = x + 3; y4 = y-3; }, 
            _ => {}
        }

        let point_vec = vec![self.data[x][y], self.data[x2][y2], self.data[x3][y3], self.data[x4][y4]];

        return point_vec.iter().fold(1, |prod, x| prod*x);
    }

    fn get_max_product(&self, x:usize, y:usize) -> u32 {
        // Right product can be calculated if
        // x <= width - 4
        // Left product can be calculated if
        // x >= 3
        // Bottom product can be calculated if
        // y <= height - 4
        // Right bottom diagonal product can be
        // calculated if x<=width -4 and
        // y <= height - 4
        // Right up diagonal product can be
        // calculated if x<=width -4 and
        // y >= 3
        let (mut rp, mut lp, mut bp, mut rbdp, mut lbdp) = (0,0,0,0,0);

        if x<=self.width - 4 {
            rp = self.get_product(x, y, "right");
        }
        if x>=3 {
            lp = self.get_product(x, y, "left");
        }
        if y <= self.height - 4 {
            bp = self.get_product(x, y, "bottom");
        }
        if x<=self.width - 4 && y <= self.height - 4 {
            rbdp = self.get_product(x, y, "diagonal-right-bottom");
        }
        if x<=self.width - 4 && y>=3 {
            lbdp = self.get_product(x, y, "diagonal-right-top");
        }

        let vals = vec![rp, lp, bp, rbdp, lbdp];

        return *vals.iter().max().unwrap();
        
    }
}

fn main() {

    let mut mygrid = Grid { data: vec![vec![0; 20]; 20], height: 20, width: 20};
    mygrid.read("grid.txt".to_string());

    let mut prod;
    let mut largest = 0;

    for x in 0..mygrid.width {
        for y in 0..mygrid.height {
            prod = mygrid.get_max_product(x, y);
            if prod > largest {
                largest = prod;
            }
        }
    }

    println!("{}", largest);
}
