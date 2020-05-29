// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are
// 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?

use std::collections::HashMap;

fn build_d1_hash_map() -> HashMap<i32, String> {
    
    let mut d1: HashMap<i32, String> = HashMap::new();

    d1.insert(1, "one".to_string());
    d1.insert(2, "two".to_string());
    d1.insert(3, "three".to_string());
    d1.insert(4, "four".to_string());
    d1.insert(5, "five".to_string());
    d1.insert(6, "six".to_string());
    d1.insert(7, "seven".to_string());
    d1.insert(8, "eight".to_string());
    d1.insert(9, "nine".to_string());
    d1.insert(10, "ten".to_string());
    d1.insert(11, "eleven".to_string());
    d1.insert(12, "twelve".to_string());
    d1.insert(13, "thirteen".to_string());
    d1.insert(14, "fourteen".to_string());
    d1.insert(15, "fifteen".to_string());
    d1.insert(16, "sixteen".to_string());
    d1.insert(17, "seventeen".to_string());
    d1.insert(18, "eighteen".to_string());
    d1.insert(19, "nineteen".to_string());

    return d1;
}

fn build_d2_hash_map() -> HashMap<i32, String> {

    let mut d2: HashMap<i32, String> = HashMap::new();

    d2.insert(10, "ten".to_string());
    d2.insert(20, "twenty".to_string());
    d2.insert(30, "thirty".to_string());
    d2.insert(40, "fourty".to_string());
    d2.insert(50, "fifty".to_string());
    d2.insert(60, "sixty".to_string());
    d2.insert(70, "seventy".to_string());
    d2.insert(80, "eighty".to_string());
    d2.insert(90, "ninety".to_string());

    return d2;
}

// Returns word for numbers 1..19
fn word1(n:i32, d1: &HashMap<i32, String>) -> String{

    let mut s: String = "".to_string();
    s.push_str(d1.get(&n).unwrap());

    return s;
}

// Returns word for numbers 10,20,30..90
fn word2(n:i32, d2: &HashMap<i32, String>) -> String{

    let mut s: String = "".to_string();    
    s.push_str(d2.get(&n).unwrap());

    return s;
}

// Returns word for any number from 20-99
fn word3(n:i32, d1: &HashMap<i32, String>, d2: &HashMap<i32, String>) -> String {

    let n1: i32;
    let n2: i32;
    let n3: i32;    
    let mut s: String;
    let s2: String;
    
    if n<=19 {
        return word1(n, d1);
    }
    if n % 10 == 0 {
        return word2(n, d2);
    }
    else {
        n1 = n / 10;
        n2 = n % 10;

        s = word2(n1*10, d2);
        s.push_str(" ");
        s2 = word1(n2, d1);
        s.push_str(&s2);

        return s;
    }
}

// Returns word for any number from 100-1000
fn word4(n:i32, d1: &HashMap<i32, String>, d2: &HashMap<i32, String>) -> String {

    let mut s:String;
    let s2:String;
    
    let remainder: i32;
    let n1: i32;
    let n2: i32;

    // Multiples of 100
    if n % 100 == 0 {
        remainder = n/100;
        if let 1...9 = remainder {
            s = word1(remainder, d1);
            s.push_str(" hundred");

            return s;
        }

        if remainder == 10 {
            return "one thousand".to_string();
        }
    }

    // Multiples of 10
    else if n % 10 == 0 {
        n1 = n/100;
        n2 = n%100;

        s = word1(n1, d1);
        s.push_str(" hundred and ");
        s2 = word2(n2, d2);
        s.push_str(&s2);

        return s;
    }  else {
        // All other numbers
        n1 = n/100;
        n2 = n%100;

        s = word1(n1, d1);
        s.push_str(" hundred and ");
        s2 = word3(n2, d1, d2);
        s.push_str(&s2);

        return s;

    }

    return "".to_string();
                
}

// Return number in words for all numbers 1..1000
fn number_to_word(n:i32, d1: &HashMap<i32, String>, d2: &HashMap<i32, String>) -> String {

    if n<=19 {
        return word1(n, d1);
    }
    if let 20...99 = n {
        return word3(n, d1, d2);
    }
    if let 100...1000 = n {
        return word4(n, d1, d2);
    }

    return "".to_string();
}
    
fn main() {

    let d1 = build_d1_hash_map();
    let d2 = build_d2_hash_map();
    let mut word: String;
    let mut word_counts: i32 = 0;

    // For Testing
    assert!(number_to_word(84, &d1, &d2) == "eighty four");
    assert!(number_to_word(1000, &d1, &d2) == "one thousand");
    assert!(number_to_word(130, &d1, &d2) == "one hundred and thirty");
    assert!(number_to_word(287, &d1, &d2) == "two hundred and eighty seven");

    for i in 1..1001 {
        word = number_to_word(i, &d1, &d2);
        let sum = word.split(" ").fold(0, |sum, x| sum + x.len());
        word_counts += sum as i32;
    }

    println!("Word count is {}", word_counts);
}
