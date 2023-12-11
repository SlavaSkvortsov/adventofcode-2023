use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use std::char::from_digit;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn char_to_offsets(chr: char) -> Vec<(i32, i32)> {
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

    match chr {
        '|' => vec![(0, -1), (0, 1)],
        '-' => vec![(-1, 0), (1, 0)],
        'L' => vec![(0, -1), (1, 0)],
        'J' => vec![(0, -1), (-1, 0)],
        '7' => vec![(0, 1), (-1, 0)],
        'F' => vec![(0, 1), (1, 0)],
        '.' => vec![],
        'S' => vec![(0, -1), (0, 1), (-1, 0), (1, 0)],  // All directions
        _ => panic!("Unknown char: {}", chr),
    }
}


fn char_to_directions(chr: char, point: Point) -> HashSet<Point> {
    char_to_offsets(chr).iter().map(|(x, y)|
        Point { x: point.x + x, y: point.y + y }
    ).collect()
}


fn get_draft_map() -> (HashMap<Point, HashSet<Point>>, Point) {
    let mut draft_map: HashMap<Point, HashSet<Point>> = HashMap::new();
    let mut start_point = Point { x: 0, y: 0 };

    if let Ok(lines) = read_lines("./input.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(row) = line {
                for (x, chr) in row.chars().enumerate() {
                    let point = Point { x: x as i32, y: y as i32 };
                    if chr == 'S' {
                        start_point = point;
                    }
                    draft_map.insert(point, char_to_directions(chr, point));
                }
            }
        }
    }

    (draft_map, start_point)
}


fn get_map() -> (HashMap<Point, HashSet<Point>>, Point) {
    let mut map: HashMap<Point, HashSet<Point>> = HashMap::new();
    let (draft_map, start_point) = get_draft_map();

    for (point, directions) in draft_map.iter() {
        let mut new_directions = HashSet::new();
        for direction in directions.iter() {
            if draft_map.contains_key(direction) && draft_map.get(direction).unwrap().contains(point) {
                new_directions.insert(*direction);
            }
        }
        if new_directions.len() > 0 {
            map.insert(*point, new_directions);
        }
    }

    (map, start_point)
}

fn get_distance_to_other_points(map: &HashMap<Point, HashSet<Point>>, start_point: Point) -> HashMap<Point, i32> {
    let mut distances: HashMap<Point, i32> = HashMap::new();
    let mut points_to_check: Vec<(Point, i32)> = vec![(start_point, 0)];

    while points_to_check.len() > 0 {
        let (point, distance) = points_to_check.remove(0);
        if !distances.contains_key(&point) || distances.get(&point).unwrap() > &distance{
            distances.insert(point, distance);
            for next_point in map.get(&point).unwrap().iter() {
                points_to_check.push((*next_point, distance + 1));
            }
        }
    }

    distances
}



fn part_1() {
    let (map, start_point) = get_map();
    draw_map(&map);

    let distances = get_distance_to_other_points(&map, start_point);
    println!("Part 1: {}", distances.values().max().unwrap());
}

fn draw_map(map: &HashMap<Point, HashSet<Point>>) {
    let mut max_x = 0;
    let mut max_y = 0;

    for point in map.keys() {
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    for y in 0..max_y + 2 {
        for x in 0..max_x + 2 {
            let point = Point { x, y };
            if map.contains_key(&point) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}


fn part_2() {
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {}
        }
    }
}

fn main() {
    part_1();
    // part_2();
}