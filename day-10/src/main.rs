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

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
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

    // Super slow and stupid way to remove dead ends
    let mut map_changed = true;
    while map_changed {
        map_changed = false;
        let mut points_to_remove: Vec<Point> = vec![];
        for (point, directions) in map.iter() {
            if directions.len() == 1 {
                points_to_remove.push(*point);
            }
        }

        // If a point points to a point that is already removed - remove it too
        for (point, directions) in map.iter() {
            for direction in directions.iter() {
                if ! map.contains_key(&direction) {
                    points_to_remove.push(*point);
                }
            }
        }

        for point in points_to_remove.iter() {
            map.remove(point);
        }
        if points_to_remove.len() > 0 {
            map_changed = true;
        }
    }

    // We need to remove all loops that are not connected to the start point
    let mut points_to_preserve: HashSet<Point> = HashSet::new();
    let mut points_to_check: Vec<Point> = vec![start_point];
    while points_to_check.len() > 0 {
        let point = points_to_check.remove(0);
        points_to_preserve.insert(point);
        for next_point in map.get(&point).unwrap().iter() {
            if !points_to_preserve.contains(next_point) {
                points_to_check.push(*next_point);
            }
        }
    }

    let mut points_to_remove: Vec<Point> = vec![];
    for (point, _) in map.iter() {
        if !points_to_preserve.contains(point) {
            points_to_remove.push(*point);
        }
    }

    for point in points_to_remove.iter() {
        map.remove(point);
    }

    // The code is SUPER optimal, but it works


    (map, start_point)
}

fn get_distance_to_other_points(map: &HashMap<Point, HashSet<Point>>, start_point: Point) -> HashMap<Point, i32> {
    let mut distances: HashMap<Point, i32> = HashMap::new();
    let mut points_to_check: Vec<(Point, i32)> = vec![(start_point, 0)];

    while points_to_check.len() > 0 {
        let (point, distance) = points_to_check.remove(0);
        if !distances.contains_key(&point) || distances.get(&point).unwrap() > &distance {
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


fn get_borders(map: &HashMap<Point, HashSet<Point>>) -> (i32, i32) {
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

    (max_x, max_y)
}


fn get_tubes_orientation() -> HashMap<Point, Orientation> {
    let mut tubes_orientation: HashMap<Point, Orientation> = HashMap::new();

    let (map, _) = get_map();

    if let Ok(lines) = read_lines("./input.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(row) = line {
                for (x, chr) in row.chars().enumerate() {
                    let point = Point { x: x as i32, y: y as i32 };

                    if ! map.contains_key(&point) {
                        continue
                    }
                    if chr == '|' {
                        tubes_orientation.insert(point, Orientation::Vertical);
                    } else if chr == '-' {
                        tubes_orientation.insert(point, Orientation::Horizontal);
                    }
                }
            }
        }
    }
    tubes_orientation
}


fn get_tubes_directions() -> HashMap<Point, (Direction, Direction)> {
    // Only for L, J, 7, F
    let mut tubes_directions: HashMap<Point, (Direction, Direction)> = HashMap::new();


    // Fuck reusing stuff
    let (map, _) = get_map();

    if let Ok(lines) = read_lines("./input.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(row) = line {
                for (x, chr) in row.chars().enumerate() {
                    let point = Point { x: x as i32, y: y as i32 };

                    if ! map.contains_key(&point) {
                        continue
                    }

                    if chr == 'L' {
                        tubes_directions.insert(point, (Direction::North, Direction::East));
                    } else if chr == 'J' {
                        tubes_directions.insert(point, (Direction::North, Direction::West));
                    } else if chr == '7' {
                        tubes_directions.insert(point, (Direction::South, Direction::West));
                    } else if chr == 'F' {
                        tubes_directions.insert(point, (Direction::South, Direction::East));
                    }
                }
            }
        }
    }
    tubes_directions
}

fn get_crossed_tubes(
    map: &HashMap<Point, HashSet<Point>>,
    tubes_orientation: &HashMap<Point, Orientation>,
    tubes_directions: &HashMap<Point, (Direction, Direction)>,
    point: Point,
    offset: (i32, i32),
) -> i32 {
    let mut crossed_tubes = 0;
    let mut next_point = Point { x: point.x + offset.0, y: point.y + offset.1 };

    let opposite_tube_orientation = match offset.0 {
        0 => Orientation::Horizontal,
        _ => Orientation::Vertical,
    };

    let mut transition: Option<Direction> = None;

    while next_point.x >= 0 && next_point.y >= 0 && next_point.x < 1000 && next_point.y < 1000 {
        if (
            tubes_orientation.contains_key(&next_point)
                && tubes_orientation.get(&next_point).unwrap() == &opposite_tube_orientation
        ) {
            crossed_tubes += 1;
        } else if tubes_directions.contains_key(&next_point) {
            if transition.is_none() {
                let directions = tubes_directions.get(&next_point).unwrap();
                if opposite_tube_orientation == Orientation::Vertical {
                    transition = Some(directions.0);
                } else {
                    transition = Some(directions.1);
                }
            } else {
                // Second transition, if different - then the tube is crossed
                let directions = tubes_directions.get(&next_point).unwrap();
                if opposite_tube_orientation == Orientation::Vertical {
                    if transition.as_ref().unwrap() != &directions.0 {
                        crossed_tubes += 1;
                    }
                } else {
                    if transition.as_ref().unwrap() != &directions.1 {
                        crossed_tubes += 1;
                    }
                }
                transition = None;
            }
        }

        next_point = Point { x: next_point.x + offset.0, y: next_point.y + offset.1 };
    }
    crossed_tubes
}


fn check_if_point_is_inside_the_loop(
    map: &HashMap<Point, HashSet<Point>>,
    tubes_orientation: &HashMap<Point, Orientation>,
    tubes_directions: &HashMap<Point, (Direction, Direction)>,
    point: Point,
) -> bool {
    let offsets = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

    for offset in offsets.iter() {
        let crossed_tubes = get_crossed_tubes(map, tubes_orientation, tubes_directions, point, *offset);
        if crossed_tubes % 2 == 0 {
            return false;
        }
    }

    true
}

fn part_2() {
    let (map, start_point) = get_map();

    let (max_x, max_y) = get_borders(&map);
    let tubes_orientation = get_tubes_orientation();
    let tubes_directions = get_tubes_directions();

    let mut result = 0;

    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            let point = Point { x, y };
            if !map.contains_key(&point) {
                let inside_the_loop = check_if_point_is_inside_the_loop(&map, &tubes_orientation, &tubes_directions, point);
                if inside_the_loop {
                    result += 1;
                    print!("I")
                } else {
                    print!("0")
                }
            } else {
                print!("#");
            }
        }
        println!();
    }

    println!("Part 2: {}", result);
}

fn main() {
    // part_1();
    part_2();
}