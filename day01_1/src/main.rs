use std::{fs::read_to_string, str::FromStr};
use regex::Regex;

const DIGIT_NAMES: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn parse_digit(s: &str) -> Result<i32, <i32 as FromStr>::Err> {
    for i in 0..10 {
        if s == DIGIT_NAMES[i] {
            return Ok(i.try_into().unwrap());
        }
    }

    s.parse::<i32>()
}

fn main() {
    let digit_regex = "[0-9]|".to_owned() + &DIGIT_NAMES.join("|");
    let digit_regex = Regex::new(&digit_regex).unwrap();

    let mut sum = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        let mut digits = digit_regex.find_iter(line);
        
        let first_digit = digits.next().unwrap();
        let last_digit = digits.last().or(Some(first_digit)).unwrap();

        let first_digit = first_digit.as_str();
        let last_digit = last_digit.as_str();

        println!("{line} => {first_digit}, {last_digit}");

        let first_digit = parse_digit(first_digit).unwrap();
        let last_digit = parse_digit(last_digit).unwrap();
        let n = first_digit * 10 + last_digit;

        println!("{n}");

        sum += n;
    }

    println!("{sum}");
}
