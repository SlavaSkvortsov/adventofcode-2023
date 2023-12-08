use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::char::from_digit;
use std::cmp::min;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_1() {
    let mut seeds: HashSet<u64> = HashSet::new();
    let mut new_iteration_seeds: HashSet<u64> = HashSet::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                if row.len() == 0 {
                    continue;
                }

                if row.starts_with("seeds: ") {
                    let seeds_str = row.split_once(":").unwrap().1.trim();
                    new_iteration_seeds = seeds_str.split(" ").map(|seed| {
                        seed.parse::<u64>().unwrap()
                    }).collect::<HashSet<u64>>();
                    continue
                }

                if row.ends_with("map:") {
                    seeds.extend(&new_iteration_seeds);
                    new_iteration_seeds = HashSet::new();
                    continue
                }

                let mut split_row = row.split(" ");
                let dest_start = split_row.next().unwrap().parse::<u64>().unwrap();
                let source_start = split_row.next().unwrap().parse::<u64>().unwrap();
                let range_len = split_row.next().unwrap().parse::<u64>().unwrap();

                let source_end = source_start + range_len;

                seeds.retain(|seed| {
                    if seed >= &source_start && seed < &source_end {
                        new_iteration_seeds.insert(dest_start + (seed - source_start));
                        false
                    }
                    else {
                        true
                    }
                });
            }
        }
    }


    seeds.extend(&new_iteration_seeds);
    println!("Result: {}", seeds.iter().min().unwrap());
}

#[derive(Eq, Hash, PartialEq)]
struct Range {
    start: u64,
    end: u64,
}


fn shift_range(
    dest_start: u64,
    source_start: u64,
    shift: u64,
    seeds: HashSet<Range>,
) -> (HashSet<Range>, HashSet<Range>) {
    let mut shifted_seeds: HashSet<Range> = HashSet::new();
    let mut original_seeds: HashSet<Range> = HashSet::new();

    let source_end = source_start + shift;

    for seed in seeds {
        // Seed does not intersect the range
        // Seed  <---------->
        // Range                   <---->
        if seed.end < source_start || seed.start > source_end {
            original_seeds.insert(seed);
            continue;
        }

        // Seed fully inside the range
        // Seed     <---->
        // Range  <---------->
        if seed.start >= source_start && seed.end <= source_end {
            shifted_seeds.insert(Range {
                start: dest_start + (seed.start - source_start),
                end: dest_start + (seed.end - source_start),
            });
            continue;
        }

        // Seed fully outside the range
        // Seed  <---------->
        // Range    <---->
        if seed.start < source_start && seed.end > source_end {
            original_seeds.insert(Range {
                start: seed.start,
                end: source_start,
            });
            original_seeds.insert(Range {
                start: source_end,
                end: seed.end,
            });
            shifted_seeds.insert(Range {
                start: dest_start,
                end: dest_start + shift,
            });
            continue;
        }

        // Seed starts inside the range and ends outside
        // Seed      <---------->
        // Range  <---->
        if seed.start >= source_start && seed.end > source_end {
            shifted_seeds.insert(Range {
                start: dest_start + (seed.start - source_start),
                end: dest_start + shift,
            });
            original_seeds.insert(Range {
                start: source_end,
                end: seed.end,
            });
            continue;
        }

        // Seed starts outside the range and ends inside
        // Seed  <---------->
        // Range      <-------->
        if seed.start < source_start && seed.end <= source_end {
            original_seeds.insert(Range {
                start: seed.start,
                end: source_start,
            });
            shifted_seeds.insert(Range {
                start: dest_start,
                end: dest_start + (seed.end - source_start),
            });
            continue;
        }

    }

    // if ! shifted_seeds.is_empty() {
    //     let (new_original_seeds, new_shifted_seeds) = shift_range(
    //         dest_start,
    //         source_start,
    //         shift,
    //         original_seeds,
    //     );
    //     shifted_seeds.extend(new_shifted_seeds);
    //     original_seeds = new_original_seeds;
    // }

    for seed in &shifted_seeds {
        if seed.start == 0 {
            println!("{} {}", seed.start, seed.end);
        }
    }
    for seed in &original_seeds {
        if seed.start == 0 {
            println!("{} {}", seed.start, seed.end);
        }
    }
    return (original_seeds, shifted_seeds);
}



fn part_2() {
    let mut seeds: HashSet<Range> = HashSet::new();
    let mut new_iteration_seeds: HashSet<Range> = HashSet::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                if row.len() == 0 {
                    continue;
                }

                if row.starts_with("seeds: ") {
                    let seeds_str = row.split_once(":").unwrap().1.trim();
                    let mut range_start = 0;
                    let mut shift = 0;


                    for (i, seed_str) in seeds_str.split(" ").enumerate() {
                        if i % 2 == 0 {
                            range_start = seed_str.parse::<u64>().unwrap();
                        }
                        else {
                            shift = seed_str.parse::<u64>().unwrap();
                            new_iteration_seeds.insert(Range {
                                start: range_start,
                                end: range_start + shift - 1,
                            });
                        }
                    }
                    continue
                }

                if row.ends_with("map:") {
                    seeds.extend(new_iteration_seeds);
                    new_iteration_seeds = HashSet::new();
                    continue
                }

                let mut split_row = row.split(" ");
                let dest_start = split_row.next().unwrap().parse::<u64>().unwrap();
                let source_start = split_row.next().unwrap().parse::<u64>().unwrap();
                let range_len = split_row.next().unwrap().parse::<u64>().unwrap();

                let (not_shifted_seeds, shifted_seeds) = shift_range(
                    dest_start,
                    source_start,
                    range_len,
                    seeds,
                );
                new_iteration_seeds.extend(shifted_seeds);
                seeds = not_shifted_seeds;
                println!("{} {}", seeds.len(), new_iteration_seeds.len());
            }
        }
    }


    seeds.extend(new_iteration_seeds);
    for seed in &seeds {
        println!("{} {}", seed.start, seed.end);
    }
    println!("Result: {}", seeds.iter().map(|seed_range| {seed_range.start}).min().unwrap());
}

fn main() {
    // part_1();
    part_2();
}