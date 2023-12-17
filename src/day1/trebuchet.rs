
use std::fs;
use regex::Regex;

pub fn solve() -> u32 {
   
   let contents = fs::read_to_string("input-trebuchet.txt").expect("Should have been able to read input file");
    

    let reg = Regex::new(r"[A-Za-z]").unwrap();
    let mut cnt = 0;
    let lines = contents.split("\n").collect::<Vec<&str>>();

    for line in lines.iter() {
        let num_line = reg.replace_all(line, "");

        let mut tmp = 0;
        let line_length = num_line.len();

        if line_length ==  1 {
            let dig = char::to_digit(num_line.chars().nth(0).unwrap(), 10).unwrap();
            tmp = dig * 11;
        } 
        
        if line_length > 1 {
            let first_digit = char::to_digit(num_line.chars().nth(0).unwrap(), 10).unwrap();
            let last_digit = char::to_digit(num_line.chars().nth(line_length - 1).unwrap(), 10).unwrap();
            tmp = ( first_digit * 10) + last_digit;
        }

        cnt += tmp;
    }
    cnt
}