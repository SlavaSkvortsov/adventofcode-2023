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

// Optional boolean decreasing
fn extrapolate(numbers: &Vec<i64>) -> i64 {
    if numbers.len() == 1 {
        panic!("I fucked up");
    }
    println!("Numbers: {:?}", numbers);
    // Check if all numbers are 0:
    if ! numbers.iter().any(|&x| x != 0) {
        return 0
    }

    let mut new_level: Vec<i64> = Vec::new();

    for i in 0..numbers.len() - 1 {
        // abs diff
        let diff = numbers[i + 1] - numbers[i];
        new_level.push(diff);
    }

    let new_number = extrapolate(&new_level) + numbers[numbers.len() - 1];
    println!("New number: {}", new_number);
    return  new_number;
}

// Optional boolean decreasing
fn extrapolate_backwards(numbers: &Vec<i64>) -> i64 {
    if numbers.len() == 1 {
        panic!("I fucked up");
    }
    println!("Numbers: {:?}", numbers);
    // Check if all numbers are 0:
    if ! numbers.iter().any(|&x| x != 0) {
        return 0
    }

    let mut new_level: Vec<i64> = Vec::new();

    for i in 0..numbers.len() - 1 {
        // abs diff
        let diff = numbers[i + 1] - numbers[i];
        new_level.insert(0, diff);
    }

    let new_number = numbers[0] - extrapolate(&new_level);
    println!("New number: {}", new_number);
    return  new_number;
}

fn part_1() {
    let mut result: i64 = 0;
    let mut result_values: Vec<i64> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let mut numbers: Vec<i64> = row.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
                println!("Numbers: {:?}", numbers);
                let new_number = extrapolate(&mut numbers);
                result += new_number;
                result_values.push(new_number);

                // make sure it's actually extrapolating
                println!("Let's extrapolate again!!!");
                numbers.push(new_number);
                extrapolate(&mut numbers);
            }
        }
    }
    println!("Result: {}", result);
    println!("Result values: {:?}", result_values);
}



fn part_2() {
    let mut result: i64 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let mut numbers: Vec<i64> = row.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
                println!("Numbers: {:?}", numbers);
                let new_number = extrapolate_backwards(&mut numbers);
                result += new_number;
            }
        }
    }
    println!("Result: {}", result);
}

fn main() {
    part_1();
    part_2();
}