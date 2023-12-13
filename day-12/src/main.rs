use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::char::from_digit;

extern crate regex;

use regex::Regex;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn validate_template(template: String, numbers: &Vec<usize>) -> bool {
    let mut i = 0;
    let mut consecutive_hashtag_count = 0;

    for c in template.chars() {
        if c == '#' {
            consecutive_hashtag_count += 1;
        } else {
            if consecutive_hashtag_count > 0 {
                if i >= numbers.len() || numbers[i] != consecutive_hashtag_count {
                    return false;
                }
                i += 1;
                consecutive_hashtag_count = 0;
            }
        }
    }

    if consecutive_hashtag_count > 0 {
        if i >= numbers.len() || numbers[i] != consecutive_hashtag_count {
            return false;
        }
        i += 1;
    }

    if i != numbers.len() {
        return false;
    }
    true
}

fn count_possible_combinations(template: String, numbers: &Vec<usize>) -> usize {
    let question_mark_index = template.find('?');
    return if let Some(question_mark_index) = question_mark_index {
        let mut new_template = template.clone();
        new_template.replace_range(question_mark_index..question_mark_index + 1, "#");
        let mut count = count_possible_combinations(new_template, numbers);

        let mut new_template = template.clone();
        new_template.replace_range(question_mark_index..question_mark_index + 1, ".");
        count += count_possible_combinations(new_template, numbers);

        count
    } else {
        if validate_template(template, numbers) {
            1
        } else {
            0
        }
    };
}


fn part_1() {
    let mut result = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let (template, numbers_str) = row.split_once(' ').unwrap();
                let numbers: Vec<usize> = numbers_str.split(',').map(|x| x.parse::<usize>().unwrap()).collect();

                let template_sting = template.to_owned();
                ;
                let tmp_result = count_possible_combinations(template_sting, &numbers);
                println!("{}: {}", template, tmp_result);
            }
        }
    }

    println!("{}", result);
}


#[derive(Clone, Copy)]
struct Number {
    start: Option<usize>,
    end: Option<usize>,
    length: usize,
}


fn check_location(template: &String, length: usize, start: usize, end: usize) -> bool {
    if start > 0 && template.chars().nth(start - 1).unwrap() == '#' {
        return false;
    }
    if end < template.len() && template.chars().nth(end).unwrap() == '#' {
        return false;
    }
    // if any of chars in the range is a dot, return false
    return ! template[start..end].chars().any(|x| x == '.')
}


fn get_possible_locations(template: &String, length: usize, start: usize, end: usize) -> Vec<usize> {
    let mut result = Vec::new();
    let mut i = start;

    while i + length <= end {
        if check_location(template, length, i, i + length) {
            result.push(i);
        }
        i += 1;
    }
    result
}

fn better_count_possible_combinations(
    template: String,
    numbers: &Vec<Number>,
) -> usize {
    let max_unchecked_number_length = numbers.iter().map( |x|
        if x.start.is_none() {
            x.length
        } else {
            0
        }
    ).max().unwrap();
    if max_unchecked_number_length == 0 {
        if template.chars().any(|x| x == '#') {
            return 0;
        }
        println!("{}", template);
        return 1;
    }

    let max_unchecked_number_index = numbers.iter().position(|&x| x.length == max_unchecked_number_length && x.start.is_none()).unwrap();
    let max_unchecked_number = numbers[max_unchecked_number_index];

    let mut start = 0;
    let mut end = template.len();

    let mut i = max_unchecked_number_index;
    while i > 0 {
        i -= 1;
        if numbers[i].end.is_some() {
            start = numbers[i].end.unwrap();
            break;
        }
    }
    i = max_unchecked_number_index;
    while i < numbers.len() - 1 {
        i += 1;
        if numbers[i].start.is_some() {
            end = numbers[i].start.unwrap();
            break;
        }
    }

    let mut end_offset = 0;
    let mut i = max_unchecked_number_index;
    while i < numbers.len() - 1 {
        i += 1;
        if numbers[i].start.is_none() {
            end_offset += numbers[i].length + 1;
        }
        else { break }
    }
    if end < end_offset {
        return 0;
    }
    end -= end_offset;

    let mut start_offset = 0;
    let mut i = max_unchecked_number_index;
    while i > 0 {
        i -= 1;
        if numbers[i].end.is_none() {
            start_offset += numbers[i].length + 1;
        }
        else { break }
    }

    let max_unchecked_number_str = max_unchecked_number.length.to_string();

    let replace_str = match max_unchecked_number_index {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "A",
        11 => "B",
        12 => "C",
        13 => "D",
        14 => "E",
        15 => "F",
        16 => "G",
        17 => "H",
        18 => "I",
        19 => "J",
        20 => "K",
        21 => "L",
        22 => "M",
        23 => "N",
        24 => "O",
        25 => "P",
        26 => "Q",
        27 => "R",
        28 => "S",
        29 => "T",
        30 => "U",
        31 => "V",
        32 => "W",
        33 => "X",
        34 => "Y",
        35 => "Z",
        _ => panic!("Invalid number index"),
    };

    let mut result = 0;

    for location_start in get_possible_locations(&template, max_unchecked_number.length, start, end).iter() {
        let mut new_template = template.clone();
        let location_end = location_start + max_unchecked_number.length;

        new_template.replace_range(location_start..&location_end, &replace_str.repeat(max_unchecked_number.length));
        if location_start > &0 {
            new_template.replace_range(location_start - 1..*location_start, ".");
        }
        if location_end < template.len() {
            new_template.replace_range(location_end..location_end + 1, ".");
        }
        let mut new_numbers = numbers.clone();
        new_numbers[max_unchecked_number_index].start = Some(*location_start);
        new_numbers[max_unchecked_number_index].end = Some(location_end);

        result += better_count_possible_combinations(new_template, &new_numbers);
    }
    result
}


fn part_2() {
    let mut result = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let (template, numbers_str) = row.split_once(' ').unwrap();
                let numbers: Vec<Number> = numbers_str.split(',').map(|x|
                    Number {
                        length: x.parse::<usize>().unwrap(),
                        start: None,
                        end: None,
                    }
                ).collect();

                let template_sting = template.to_owned();
                let five_times_template_string = format!("{}?{}?{}?{}?{}", template, template, template, template, template);
                let mut five_times_numbers = numbers.clone();
                five_times_numbers.extend(numbers.clone());
                five_times_numbers.extend(numbers.clone());
                five_times_numbers.extend(numbers.clone());
                five_times_numbers.extend(numbers.clone());

                let tmp_result = better_count_possible_combinations(five_times_template_string, &five_times_numbers);
                println!("{}: {}", template, tmp_result);
                result += tmp_result;
            }
        }
    }

    println!("{}", result);
}

fn main() {
    // part_1();
    part_2();
}