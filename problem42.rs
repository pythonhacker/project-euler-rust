// Coded triangle numbers
use std::io::prelude::*;
use std::fs::File;

fn gen_triangle_nums(limit: u32) -> Vec<u32> {

    let mut tri_nums: Vec<u32> = vec![];

    for i in 1..limit+1 {
        tri_nums.push(i*(i+1)/2);
    }

    return tri_nums;
}

fn read_words(filename: &str) -> Vec<String> {

    let mut file = File::open(filename).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let words: Vec<String> = contents.split(",").map( |s| s.to_string().replace("\"", "")).collect();
    //    println!("{:?}", words);

    return words;
}

fn word_value(word: String) -> u32 {

    let alpha: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut idx: usize;

    let word_vec: Vec<char> = word.chars().collect();
    let mut word_val: usize = 0;
        
    for c in word_vec.iter() {
        idx = alpha.iter().position( |r| *r == *c).unwrap() + 1;
        word_val += idx;
    }

    return word_val as u32;
}

fn find_triangle_words(filename: &str) -> u32 {

    let words: Vec<String> = read_words(filename);
    let tri_nums = gen_triangle_nums(100);
    let mut count:u32 = 0;
    
    for word in words.iter() {
        let w_val: u32 = word_value(word.to_string());
        println!("{} {}", word, w_val);

        if tri_nums.iter().position( |r| *r == w_val) != None {
            count += 1;
        }
    }
    
    return count;
}


fn main() {
    println!("{}", find_triangle_words("triangle_words.txt"));
}
