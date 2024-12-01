use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn one(filename: &String) {
    // Part one
    let mut lists = vec![Vec::<u64>::new(); 2];
    let file = File::open(filename.as_str()).unwrap();
    let reader = BufReader::new(file);
    // Read the location IDs from both lists
    for line in reader.lines().map(|l| l.unwrap()) {
        let loc_ids = line.split_whitespace().take(2);
        for (ind, loc_id) in loc_ids.enumerate() {
            lists[ind].push(u64::from_str(loc_id).unwrap());
        }
    }
    // Sort in ascending order
    lists[0].sort();
    lists[1].sort();
    // Compute the difference for each entry
    let sum: u64 = lists[0]
        .iter()
        .zip(lists[1].iter())
        .map(|(v0, v1)| if v0 > v1 { v0 - v1 } else { v1 - v0 })
        .sum();
    println!("Sum of differences = {}", sum);
    // Part two
    // Count frequency of each location ID in the right list
    let mut counter_right = HashMap::new();
    for loc_id in lists[1].iter() {
        match counter_right.get_mut(loc_id) {
            Some(count) => {
                *count += 1u64;
            }
            None => {
                counter_right.insert(loc_id, 1u64);
            }
        }
    }
    // Compute the similarity score
    let mut sim_score = 0u64;
    for loc_id in lists[0].iter() {
        match counter_right.get(loc_id) {
            Some(count) => {
                sim_score += loc_id * count;
            }
            None => {}
        }
    }
    println!("Similarity score = {}", sim_score);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        let arg = args[1].as_str();
        let day = u64::from_str(arg).unwrap();
        match day {
            1 => {
                one(&args[2]);
            }
            _ => println!("Unknown day {}", day),
        }
    }
}
