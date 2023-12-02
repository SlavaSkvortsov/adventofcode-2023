use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::char::from_digit;
use std::cmp;



struct Round {
    red: u8,
    green: u8,
    blue: u8,
}


struct Game {
    id: u64,
    rounds: Vec<Round>,
}

fn parse_row(row: String) -> Game {
    // Line example:
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

    let (game_id_part, rounds_part) = row.split_once(":").unwrap();
    let game_id = game_id_part.split_once(" ").unwrap().1.parse::<u64>().unwrap();

    let rounds = rounds_part.split(";").map(|round| {
        let colors: HashMap<&str, u8> = round.split(",").map(|color| {
            let color = color.trim();
            let (color_count, color_name) = color.split_once(" ").unwrap();
            let int_color_count = color_count.parse::<u8>().unwrap();
            (color_name, int_color_count)
        }).collect();
        Round {
            red: *colors.get("red").unwrap_or(&0),
            green: *colors.get("green").unwrap_or(&0),
            blue: *colors.get("blue").unwrap_or(&0),
        }
    }).collect();
    Game {
        id: game_id,
        rounds,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn part_1() {
    // 12 red cubes, 13 green cubes, and 14 blue
    let max_cubes = Round {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut result: u64 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let game = parse_row(row);
                let game_failed = game.rounds.iter().any(|round| {
                    round.red > max_cubes.red || round.green > max_cubes.green || round.blue > max_cubes.blue
                });

                if ! game_failed {
                    result += game.id;
                }
            }
        }
    }

    println!("{}", result);
}



fn part_2() {
    // 12 red cubes, 13 green cubes, and 14 blue
    let mut result: u64 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let game = parse_row(row);
                let mut min_cubes = Round {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                for round in game.rounds.iter() {
                    min_cubes.red = cmp::max(round.red, min_cubes.red);
                    min_cubes.green = cmp::max(round.green, min_cubes.green);
                    min_cubes.blue = cmp::max(round.blue, min_cubes.blue);
                }

                result += min_cubes.red as u64 * min_cubes.green as u64 * min_cubes.blue as u64;
            }
        }
    }

    println!("{}", result);
}

fn main() {
    part_1();
    part_2();
}