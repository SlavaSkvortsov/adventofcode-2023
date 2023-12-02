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
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {

            }
        }
    }
}



fn part_2() {
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {

            }
        }
    }
}

fn main() {
    part_1();
    part_2();
}