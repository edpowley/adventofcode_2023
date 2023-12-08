use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let digit_regex = Regex::new("[0-9]").unwrap();

    let mut sum = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        let mut digits = digit_regex.find_iter(line);
        let first_digit = digits.next().unwrap();
        let last_digit = digits.last().or(Some(first_digit)).unwrap();
        let first_digit = first_digit.as_str().parse::<i32>().unwrap();
        let last_digit = last_digit.as_str().parse::<i32>().unwrap();
        sum += first_digit * 10 + last_digit;
    }

    println!("{sum}");
}
