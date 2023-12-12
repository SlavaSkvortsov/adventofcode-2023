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

struct Point {
    x: usize,
    y: usize,
}


fn part_1_not_working() {
    let mut data: Vec<Vec<bool>> = Vec::new();
    let mut points: Vec<Point> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(row) = line {
                let mut row_data: Vec<bool> = Vec::new();
                for (x, c) in row.chars().enumerate() {
                    if c == '#' {
                        row_data.push(true);
                        points.push(Point{x, y});
                    } else {
                        row_data.push(false);
                    }
                }
                data.push(row_data);
            }
        }
    }

    println!("{:?}", data);

    let mut space_expand_rows: Vec<usize> = Vec::new();

    for (y, row) in data.iter().enumerate() {
        if ! row.iter().any(|&x| x) {
            space_expand_rows.push(y);
        }
    }
    println!("{:?}", space_expand_rows);

    let mut space_expand_cols: Vec<usize> = Vec::new();

    for x in 0..data[0].len() {
        let mut expanded = true;
        for y in 0..data.len() {
            if data[y][x] {
                expanded = false;
                break
            }
        }
        if expanded {
            space_expand_cols.push(x);
        }
    }

    println!("{:?}", space_expand_cols);

    let mut total_distance = 0;

    for (i, point) in points.iter().enumerate() {
        for another_point in points[i+1..].iter() {
            let distance = ((point.x as i32 - another_point.x as i32).abs() + (point.y as i32 - another_point.y as i32).abs()) as usize;

            let space_expanded_columns = space_expand_cols.iter().filter(|&x|
                x >= &point.x && x <= &another_point.x
                || x <= &point.x && x >= &another_point.x
            ).count();
            let space_expanded_rows = space_expand_rows.iter().filter(
                |&y| y >= &point.y && y <= &another_point.y
                || y <= &point.y && y >= &another_point.y
            ).count();

            let final_distance = distance + (space_expanded_columns + space_expanded_rows) * (1000000 - 1);
            total_distance += final_distance;

            // println!("{} {} {} {} {}", point.x, point.y, another_point.x, another_point.y, final_distance)

        }
    }

    println!("{}", total_distance);

}

fn part_1() {
    let mut data: Vec<Vec<bool>> = Vec::new();
    let mut points: Vec<Point> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(row) = line {
                let mut row_data: Vec<bool> = Vec::new();
                for (x, c) in row.chars().enumerate() {
                    if c == '#' {
                        row_data.push(true);
                    } else {
                        row_data.push(false);
                    }
                }
                data.push(row_data);
            }
        }
    }

    println!("{:?}", data);

    let mut space_expand_rows: Vec<usize> = Vec::new();

    for (y, row) in data.iter().enumerate() {
        if ! row.iter().any(|&x| x) {
            space_expand_rows.push(y);
        }
    }
    println!("{:?}", space_expand_rows);
    space_expand_rows.sort();
    space_expand_rows.reverse();

    let x_len = data[0].len();
    for y in space_expand_rows.iter() {
        data.insert(*y, vec![false; x_len]);
    }

    let mut space_expand_cols: Vec<usize> = Vec::new();

    for x in 0..data[0].len() {
        let mut expanded = true;
        for y in 0..data.len() {
            if data[y][x] {
                expanded = false;
                break
            }
        }
        if expanded {
            space_expand_cols.push(x);
        }
    }

    space_expand_cols.sort();
    space_expand_cols.reverse();

    for x in space_expand_cols.iter() {
        for row in data.iter_mut() {
            row.insert(*x, false);
        }
    }

    let mut total_distance = 0;

    for (y, row) in data.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if *point {
                points.push(Point{x, y});
            }
        }
    }

    for (i, point) in points.iter().enumerate() {
        for another_point in points[i+1..].iter() {
            let distance = ((point.x as i32 - another_point.x as i32).abs() + (point.y as i32 - another_point.y as i32).abs()) as usize;
            total_distance += distance;
            println!("{} {} {} {} {}", point.x, point.y, another_point.x, another_point.y, distance)
        }
    }

    println!("{}", total_distance);

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
    part_1_not_working();
    part_2();
}