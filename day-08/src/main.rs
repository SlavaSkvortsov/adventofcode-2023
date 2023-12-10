use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::char::from_digit;
extern crate divisors;
use divisors::get_divisors;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_instructions(instructions: String) -> Vec<usize> {
    return instructions.chars().map(|c| {
        if c == 'R' {
            1_usize
        }
        else {
            0_usize
        }
    }).collect::<Vec<usize>>();
}


fn get_steps(instructions: &Vec<usize>, current_node: &String, map: &HashMap<String, (String, String)>)  -> usize {
    let mut previous_stop = 0;
    let mut steps = 0;
    let mut iterations = 0;
    let mut instruction_index: usize = 0;
    let mut current_node = current_node.to_string();

    while iterations < 50 {
        let instruction = instructions[instruction_index];
        let (left, right) = map.get(&current_node).unwrap();
        let next = if instruction == 0 {
            left
        }
        else {
            right
        };
        current_node = next.to_string();
        steps += 1;
        instruction_index += 1;
        if instruction_index >= instructions.len() {
            instruction_index = 0;
        }
        if current_node.ends_with("Z") {
            return steps;
        }
    }
    return steps;
}


fn part_1() {
    let mut instructions: Vec<usize> = Vec::new();
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                if instructions.len() == 0 {
                    instructions = read_instructions(row);
                    continue
                }

                if row.len() == 0 {
                    continue
                }

                // row example:
                // AAA = (BBB, CCC)
                let (from, directions) = row.split_once(" = ").unwrap();
                let (left, right) = directions[1..directions.len() - 1].split_once(", ").unwrap();
                map.insert(from.to_string(), (left.to_string(), right.to_string()));
            }
        }
    }

    let mut current_nodes: Vec<String> = map.keys().cloned().collect::<Vec<String>>();

    // Keep only nodes that end with A
    current_nodes = current_nodes.iter().filter(|n| n.ends_with("A")).map(|n| n.to_string()).collect::<Vec<String>>();

    struct Node {
        name: String,
        distance: usize,
    }

    let mut steps = 0;
    let mut instruction_index: usize = 0;

    let mut result: usize = 1;

    for node in current_nodes {
        let mut steps = get_steps(&instructions, &node, &map);
        let divisors = get_divisors(steps as u64);
        result *= divisors[0] as usize;
        println!("Steps: {}, Node: {}, Divisors: {:?}", steps, node, divisors);
    }
    result *= 277;
    println!("Result: {}", result);
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