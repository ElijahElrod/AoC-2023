use regex::Regex;
use std::fs;
use substring::Substring;

pub fn _solve() -> u32 {
    let contents = fs::read_to_string("input-trebuchet.txt")
        .expect("Should have been able to read input file");

    let reg = Regex::new(r"[A-Za-z]").unwrap();
    let mut cnt = 0;
    let lines = contents.split("\n").collect::<Vec<&str>>();

    for line in lines.iter() {
        let num_line = reg.replace_all(line, "");

        let mut tmp = 0;
        let line_length = num_line.len();

        if line_length >= 1 {
            let first_digit = char::to_digit(num_line.chars().nth(0).unwrap(), 10).unwrap();
            let last_digit =
                char::to_digit(num_line.chars().nth(line_length - 1).unwrap(), 10).unwrap();
            tmp = (first_digit * 10) + last_digit;
        }

        cnt += tmp;
    }
    cnt
}

pub fn _solve_part_2() -> u32 {
    let contents = fs::read_to_string("input-trebuchet.txt")
        .expect("Should have been able to read input file");

    let mut cnt = 0;
    let reg = Regex::new(r"[A-Za-z]").unwrap();
    let lines = contents.split("\n").collect::<Vec<&str>>();

    for line in lines.iter() {
        let mut replaced_line = " ".to_owned();

        for (ind, c) in line.chars().enumerate() {
            match c {
                'o' => {
                    let sub_str = line.substring(ind, ind + 3);
                    if sub_str == "one" {
                        replaced_line.push_str("1");
                    }
                }
                't' => match line.chars().nth(ind + 1).unwrap_or_default() {
                    'w' => {
                        let sub_str = line.substring(ind, ind + 3);
                        if sub_str == "two" {
                            replaced_line.push_str("2");
                        }
                    }
                    'h' => {
                        let sub_str = line.substring(ind, ind + 5);
                        if sub_str == "three" {
                            replaced_line.push_str("3");
                        }
                    }
                    _ => continue,
                },
                'f' => match line.chars().nth(ind + 1).unwrap_or_default() {
                    'o' => {
                        let sub_str = line.substring(ind, ind + 4);
                        if sub_str == "four" {
                            replaced_line.push_str("4");
                        }
                    }
                    'i' => {
                        let sub_str = line.substring(ind, ind + 4);
                        if sub_str == "five" {
                            replaced_line.push_str("5")
                        }
                    }
                    _ => continue,
                },
                's' => match line.chars().nth(ind + 1).unwrap_or_default() {
                    'i' => {
                        let sub_str = line.substring(ind, ind + 3);
                        if sub_str == "six" {
                            replaced_line.push_str("6")
                        }
                    }
                    'e' => {
                        let sub_str = line.substring(ind, ind + 5);
                        if sub_str == "seven" {
                            replaced_line.push_str("7")
                        }
                    }
                    _ => continue,
                },
                'e' => {
                    let sub_str = line.substring(ind, ind + 5);
                    if sub_str == "eight" {
                        replaced_line.push_str("8")
                    }
                }
                'n' => {
                    let sub_str = line.substring(ind, ind + 4);
                    if sub_str == "nine" {
                        replaced_line.push_str("9")
                    }
                }
                _ => {
                    replaced_line.push_str(&c.to_string());
                }
            }
        }

        let num_line = reg.replace_all(replaced_line.as_str().trim(), "");

        let mut tmp = 0;
        let line_length = num_line.len();

        if line_length >= 1 {
            let first_digit =
                char::to_digit(num_line.chars().nth(0).unwrap(), 10).unwrap_or_default();
            let last_digit = char::to_digit(num_line.chars().nth(line_length - 1).unwrap(), 10)
                .unwrap_or_default();
            tmp = (first_digit * 10) + last_digit;
            
        }

        cnt += tmp;
    }
    cnt
}
