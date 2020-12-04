use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines(file_name: &'static str) -> Vec<String> {
    let path = Path::new(file_name);

    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut result = vec![];
    for line in lines {
        let line = line.unwrap();
        result.push(line);
    }

    result
}

fn main() {
    println!("Test input:");
    let lines = read_lines("test_input.txt");
    check_input(&lines, true);
    check_input(&lines, false);

    println!("Actual input:");
    let lines = read_lines("input.txt");
    check_input(&lines, true);
    check_input(&lines, false);
}

fn check_input(lines: &Vec<String>, is_part_one: bool) {
    let mut valid_passwords = 0;
    for line in lines {
        if is_valid_password(line.clone(), is_part_one) {
            valid_passwords += 1;
        }
    }

    if is_part_one {
        println!("Valid passwords for part 1: {:?}.", valid_passwords);
    } else {
        println!("Valid passwords for part 2: {:?}.", valid_passwords);
    }
}

fn is_valid_password(line: String, is_part_one: bool) -> bool {
    let line = line.trim();

    let split_whitespace: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();

    let (min, max) = {
        let split_nums: Vec<i32> = split_whitespace[0]
            .split("-")
            .map(|s| s.to_string())
            .map(|n| n.parse().unwrap())
            .collect();
        let min = split_nums[0];
        let max = split_nums[1];

        (min, max)
    };

    let matching_char = { split_whitespace[1].chars().nth(0).unwrap() };
    let password = split_whitespace[2].clone();
    if is_part_one {
        let mut matching_count = 0;
        for c in password.chars() {
            if c == matching_char {
                matching_count += 1;
            }
        }

        return matching_count >= min && matching_count <= max;
    } else {
        let mut first_is_valid = false;
        let mut second_is_valid = false;
        for (i, c) in password.chars().enumerate() {
            let i = (i + 1) as i32;
            let matching = c == matching_char;

            if i == min {
                first_is_valid = matching;
            }

            if i == max {
                second_is_valid = matching;
            }
        }

        if first_is_valid && second_is_valid {
            return false;
        }

        if !first_is_valid && !second_is_valid {
            return false;
        }

        return (first_is_valid && !second_is_valid) || (!first_is_valid && second_is_valid);
    }

    false
}
