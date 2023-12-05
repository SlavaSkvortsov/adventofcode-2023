use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::char::from_digit;
use std::collections::VecDeque;
use std::cmp;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


struct Line {
    winning: HashSet<u64>,
    having: HashSet<u64>,
}


fn parse_numbers(numbers: &str) -> HashSet<u64> {
    numbers.replace("  ", " ").trim().split(" ").map(|number| {
        number.parse::<u64>().unwrap()
    }).collect::<HashSet<u64>>()
}


fn read_line(line: String) -> Line {
    // Line example:
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let numbers = line.split_once(":").unwrap().1.trim();
    let (winning, having) = numbers.split_once("|").unwrap();
    Line {
        winning: parse_numbers(winning),
        having: parse_numbers(having),
    }
}

fn part_1() {
    let mut result: u32 = 0;
    let mut additional_cards: VecDeque<u64> = VecDeque::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let line = read_line(row);
                let common_count = line.winning.intersection(&line.having).count();

                let won_additional = additional_cards.pop_front().unwrap_or(0);
                result += (won_additional + 1) as u32;

                if common_count > 0 {

                    for i in 0..cmp::min(common_count, additional_cards.len()) {
                        *additional_cards.get_mut(i).unwrap() += 1 + won_additional;
                    }
                    while additional_cards.len() < common_count {
                        additional_cards.push_back(1 + won_additional);
                    }
                }
            }
        }
    }

    println!("{}", result);
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
    // part_2();
}