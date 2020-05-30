// Names scores
// Please read the original problem.
use std::io::prelude::*;
use std::fs::File;

fn get_index(letter: String) -> usize  {
    let alpha: Vec<String> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().map( |s| s.to_string()).collect();
    return alpha.iter().position( |r| *r == letter ).unwrap() + 1;
}

fn get_score(name: &String) ->usize {
    
    let sum = name.chars().map( |s| get_index(s.to_string()) ).fold(0, |sum, x| sum + x);
//    println!("{}", sum);
    return sum;
}

fn read_names(filename: &str)  {

    let mut file = File::open(filename).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let mut names: Vec<String> = contents.split(",").map( |s| s.to_string().replace("\"", "")).collect();
    names.sort();
    
    //    println!("{:?}", names);
    let mut idx: usize = 1;
    let mut score: usize;
    let mut scores: usize = 0;
    
    for name in names.iter() {
        score = get_score(name)*idx;
        println!("Score for {} is {}", *name, score);
        scores += score;
        idx += 1;
    }

    println!("Total Score => {}", scores);
}

fn main() {

    read_names("names.txt");
}
