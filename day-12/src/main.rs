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
    }
}



fn part_1() {
    let mut result = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let (template, numbers_str) = row.split_once(' ').unwrap();
                let numbers: Vec<usize> = numbers_str.split(',').map(|x| x.parse::<usize>().unwrap()).collect();

                let template_sting = template.to_owned();;
                result += count_possible_combinations(template_sting, &numbers);
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
    part_2();
}