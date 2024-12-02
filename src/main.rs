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
        .map(|(v0, v1)| v0.abs_diff(*v1))
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

fn two(filename: &String) {
    // Read the reports
    let file = File::open(filename.as_str()).unwrap();
    let reader = BufReader::new(file);
    // Part one: Count the number of safe reports
    let mut reports = vec![];
    for line in reader.lines().map(|l| l.unwrap()) {
        let row = line
            .split_whitespace()
            .map(|r| u64::from_str(r).unwrap())
            .collect::<Vec<u64>>();
        if row.len() > 1 {
            reports.push(row);
        }
    }
    let is_safe_report = move |row: &Vec<u64>| {
        // Check ordering from the first two entries
        let asc_start = row[0] < row[1];
        let mut safe = true;
        for i in 1..row.len() {
            let rep0 = row[i - 1];
            let rep1 = row[i];
            let diff = rep0.abs_diff(rep1);
            let asc = rep0 < rep1;
            if diff == 0 || diff > 3 || asc != asc_start {
                safe = false;
            }
        }
        safe
    };
    let mut n_safe_reports = 0;
    for row in reports.iter() {
        if is_safe_report(&row) == true {
            n_safe_reports += 1;
        }
    }
    println!("Number of safe reports = {}", n_safe_reports);
    // Part two: Count the number of safe reports but one report can be dropped
    let mut n_safe_reports_skipped = 0;
    for row in reports.iter() {
        if is_safe_report(&row) == true {
            n_safe_reports_skipped += 1;
        } else {
            // To avoid unnecessary allocations copy elements into a temporary vec
            let n_reports_row = row.len();
            let mut tmp_row = vec![0u64; n_reports_row - 1];
            for ind_skip in 0..n_reports_row {
                tmp_row[0..ind_skip].copy_from_slice(&row[0..ind_skip]);
                tmp_row[ind_skip..].copy_from_slice(&row[ind_skip + 1..]);
                if is_safe_report(&tmp_row) == true {
                    n_safe_reports_skipped += 1;
                    break;
                }
            }
        }
    }
    println!(
        "Number of safe reports skipping one entry = {}",
        n_safe_reports_skipped
    );
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
            2 => {
                two(&args[2]);
            }
            _ => println!("Unknown day {}", day),
        }
    }
}
