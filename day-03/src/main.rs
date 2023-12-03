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


#[derive(Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

struct Number {
    value: u64,
    id: u64,
    points: Vec<Point>,
}


fn main() {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: HashMap<Point, char> = HashMap::new();
    let mut next_id: u64 = 0;


    if let Ok(lines) = read_lines("./input.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(row) = line {
                let mut chars = row.chars();
                let mut digits: Vec<char> = Vec::new();

                let mut x = 0;
                loop {
                    let mut char = chars.next();
                    x += 1;

                    if (char.is_none() | (!char.unwrap_or('.').is_digit(10))) & !digits.is_empty() {
                        let points: Vec<Point> = (0..digits.len()).map(|i| {
                            Point {
                                x: x - digits.len() + i,
                                y,
                            }
                        }).collect();

                        let value = digits.iter().collect::<String>().parse().unwrap();
                        numbers.push(
                            Number {
                                id: next_id,
                                value,
                                points,
                            }
                        );

                        next_id += 1;
                        digits.clear();
                    }
                    if char == None {
                        break;
                    }
                    let char_unwrapped = char.unwrap();
                    if char_unwrapped.is_digit(10) {
                        digits.push(char_unwrapped);
                        continue;
                    } else if char_unwrapped == '.' {
                        continue;
                    } else {
                        symbols.insert(Point { x, y }, char_unwrapped);
                    }
                }
            }
        }
    }

    let mut numbers_map: HashMap<&Point, &Number> = numbers.iter().flat_map(|number| {
        number.points.iter().map(move |point| {
            (point, number)
        })
    }).collect();


    // let mut selected_numbers: HashMap<u64, &Number> = HashMap::new();
    // for point in symbols.keys() {
    //     for i in 0..3 {
    //         for j in 0..3 {
    //             let point = Point {
    //                 x: point.x + i - 1,
    //                 y: point.y + j - 1,
    //             };
    //             if let Some(number) = numbers_map.get(&point) {
    //                 selected_numbers.insert(number.id, &number);
    //             }
    //         }
    //     }
    // }
    //
    // let result: u64 = selected_numbers.values().map(|number| number.value).sum();
    // println!("part 1 - {}", result);


    let mut result: u64 = 0;

    for (point, symbol) in symbols.into_iter() {
        if symbol != '*' {
            continue;
        }

        let mut selected_numbers: HashMap<u64, &Number> = HashMap::new();
        for i in 0..3 {
            for j in 0..3 {
                let point = Point {
                    x: point.x + i - 1,
                    y: point.y + j - 1,
                };
                if let Some(number) = numbers_map.get(&point) {
                    selected_numbers.insert(number.id, &number);
                }
            }
        }

        if selected_numbers.len() == 2 {
            result += selected_numbers.values().map(|number| number.value).product::<u64>();
        }

    }

    println!("part 2 - {}", result);
}
