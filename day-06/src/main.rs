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

struct Input {
    time: u64,
    distance: u64,
}


fn get_distance(total_time: u64, time_to_charge: u64) -> u64 {
    let time_to_travel = total_time - time_to_charge;
    return time_to_travel * time_to_charge;
}

// time_to_charge is unknown
// Formula:
// (total_time - time_to_charge) * time_to_charge > required_distance
// total_time * time_to_charge - time_to_charge^2 > required_distance
// time_to_charge^2 - total_time * time_to_charge + required_distance < 0
// time_to_charge^2 - total_time * time_to_charge + required_distance = 0
// time_to_charge = (total_time +- sqrt(total_time^2 - 4 * required_distance)) / 2

fn get_time_to_charge(total_time: u64, required_distance: u64) -> (u64, u64) {
    let a = 1;
    let b = total_time as u64;
    let c = required_distance as i64;
    let discriminant: i64 = b.pow(2) as i64 - 4 * a * c;
    if discriminant < 0 {
        panic!("Discriminant is negative");
    }
    let sqrt_discriminant = (discriminant as f64).sqrt() as u64;
    let time_to_charge_1: f64 = (total_time as i64 - sqrt_discriminant as i64) as f64 / 2_f64;
    let time_to_charge_2: f64 = (total_time as i64 + sqrt_discriminant as i64) as f64 / 2_f64;

    let mut time_to_charge_start: u64 = time_to_charge_1.ceil() as u64;
    let mut time_to_charge_stop: u64 = time_to_charge_2.floor() as u64;

    if get_distance(total_time, time_to_charge_start as u64) <= required_distance {
        time_to_charge_start += 1;
    }
    if get_distance(total_time, time_to_charge_stop as u64) <= required_distance {
        time_to_charge_stop -= 1;
    }

    return (time_to_charge_start, time_to_charge_stop);
}


fn part_1() {
    // Test data:
    // Time:      7  15   30
    // Distance:  9  40  200
    // let inputs = vec![
    //     Input { time: 7, distance: 9 },
    //     Input { time: 15, distance: 40 },
    //     Input { time: 30, distance: 200 },
    // ];

    // real data:
    // Time:        53     89     76     98
    // Distance:   313   1090   1214   1201
    // let inputs = vec![
    //     Input { time: 53, distance: 313 },
    //     Input { time: 89, distance: 1090 },
    //     Input { time: 76, distance: 1214 },
    //     Input { time: 98, distance: 1201 },
    // ];

    // test data part 2:
    // Time:      71530
    // Distance:  940200
    // let inputs = vec![
    //     Input { time: 71530, distance: 940200 },
    // ];

    // real data part 2:
    // Time:        53897698
    // Distance:   313109012141201
    let inputs = vec![
        Input { time: 53897698, distance: 313109012141201 },
    ];

    let mut result: u64 = 0;

    for input in inputs {
        let (time_to_charge_1, time_to_charge_2) = get_time_to_charge(input.time, input.distance);
        println!("Time to charge 1: {}", time_to_charge_1);
        println!("Time to charge 2: {}", time_to_charge_2);
        println!("");

        if result == 0 {
            result = time_to_charge_2 - time_to_charge_1 + 1;
            continue;
        }
        result *= time_to_charge_2 - time_to_charge_1 + 1;
    }

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