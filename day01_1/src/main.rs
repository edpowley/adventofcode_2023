use std::fs::read_to_string;

const DIGIT_NAMES: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn find_first_digit(s: &str) -> Option<(u32, usize)> {
    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();
        if c.is_numeric() {
            return Some((c.to_digit(10).unwrap(), i));
        }

        for n in 0..10 {
            if s[i..].starts_with(DIGIT_NAMES[n]) {
                return Some((n.try_into().unwrap(), i));
            }
        }
    }

    None
}

fn main() {
    let mut sum = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        let (first_digit, mut i) = find_first_digit(line).unwrap();
        let mut last_digit = first_digit;

        while let Some((next_digit, next_i)) = find_first_digit(&line[i+1..]) {
            last_digit = next_digit;
            i += next_i + 1;
        }

        println!("{line} => {first_digit}, {last_digit}");

        let n = first_digit * 10 + last_digit;
        sum += n;
    }

    println!("{sum}");
}
