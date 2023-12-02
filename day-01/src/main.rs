use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::char::from_digit;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_1() {
    let mut result = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let mut first_digit: Option<char> = None;
                let mut last_digit: Option<char> = None;
                for c in row.chars() {
                    if c.is_digit(10) {
                        if first_digit.is_none() {
                            first_digit = Some(c);
                        }
                        last_digit = Some(c);
                    }
                }
                result += format!("{}{}", first_digit.unwrap(), last_digit.unwrap()).parse::<i32>().unwrap();
            }
        }
    }

    println!("{}", result);
}



fn part_2() {

    let text_to_digit = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("six", 6),
        ("four", 4),
        ("five", 5),
        ("nine", 9),
        ("three", 3),
        ("seven", 7),
        ("eight", 8),
    ]);
    let mut result = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let mut first_digit: Option<char> = None;
                let mut last_digit: Option<char> = None;
                let mut pointer: usize = 0;

                while pointer < row.len() {
                    let c = row.chars().nth(pointer).unwrap();
                    if c.is_digit(10) {
                        if first_digit.is_none() {
                            first_digit = Some(c);
                        }
                        last_digit = Some(c);
                    }
                    else if c.is_alphabetic() {
                        for length in 3..=5 {
                            if pointer + length > row.len() {
                                break;
                            }
                            let mut word: String = String::new();
                            for i in 0..length {
                                word.push(row.chars().nth(pointer + i).unwrap());
                            }
                            let digit = text_to_digit.get(&*word);
                            if let Some(ok_digit) = digit {
                                if first_digit.is_none() {
                                    first_digit = from_digit(*ok_digit, 10);
                                }
                                last_digit = from_digit(*ok_digit, 10);
                                break;
                            }
                        }
                    }
                    pointer += 1;
                }
                result += format!("{}{}", first_digit.unwrap(), last_digit.unwrap()).parse::<i32>().unwrap();
            }
        }
    }

    println!("{}", result);
}

fn main() {
    part_1();
    part_2();
}