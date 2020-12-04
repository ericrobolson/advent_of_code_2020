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
    let lines = read_lines("input.txt");
    let numbers: Vec<i32> = lines.iter().map(|s| s.parse().unwrap()).collect();

    let target_sum = 2020;
    match get_two_numbers_that_match_sum(target_sum, &numbers) {
        Some((n1, n2)) => {
            println!(
                "Matching numbers that sum to: {:?} are ({:?}, {:?}).",
                target_sum, n1, n2
            );

            println!("Product: {:?}", n1 * n2);
        }
        None => {
            println!("No matching numbers!");
        }
    }

    match get_three_numbers_that_match_sum(target_sum, &numbers) {
        Some((n1, n2, n3)) => {
            println!(
                "Matching numbers that sum to: {:?} are ({:?}, {:?}, {:?}).",
                target_sum, n1, n2, n3
            );

            println!("Product: {:?}", n1 * n2 * n3);
        }
        None => {
            println!("No matching numbers!");
        }
    }
}

fn get_two_numbers_that_match_sum(target_sum: i32, numbers: &Vec<i32>) -> Option<(i32, i32)> {
    for number1 in numbers {
        for number2 in numbers {
            let number1 = *number1;
            let number2 = *number2;
            if number1 == number2 {
                continue;
            }

            let sum = number1 + number2;
            if target_sum == sum {
                return Some((number1, number2));
            }
        }
    }

    None
}

fn get_three_numbers_that_match_sum(
    target_sum: i32,
    numbers: &Vec<i32>,
) -> Option<(i32, i32, i32)> {
    for number1 in numbers {
        for number2 in numbers {
            for number3 in numbers {
                let number1 = *number1;
                let number2 = *number2;
                let number3 = *number3;
                if number1 == number2 && number2 == number3 {
                    continue;
                }

                let sum = number1 + number2 + number3;
                if target_sum == sum {
                    return Some((number1, number2, number3));
                }
            }
        }
    }

    None
}
