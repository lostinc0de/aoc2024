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

fn three(filename: &String) {
    let file = File::open(filename.as_str()).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    // Part one: Add up valid mul() instructions
    // This lambda evaluates the mul() instruction at a given position
    let mul = move |pos: usize, line: &String| -> u64 {
        let start = pos + 4;
        let comma = match line[start..].find(",") {
            Some(pos_comma) => start + pos_comma,
            None => {
                return 0u64;
            }
        };
        let end = match line[start..].find(")") {
            Some(pos_end) => start + pos_end,
            None => {
                return 0u64;
            }
        };
        if start < comma && comma < end {
            let mul0 = u64::from_str(&line[start..comma]).unwrap_or(0u64);
            let mul1 = u64::from_str(&line[comma + 1..end]).unwrap_or(0u64);
            return mul0 * mul1;
        }
        return 0u64;
    };
    let mut sum = 0u64;
    for line in lines.iter() {
        for (pos, _) in line.match_indices("mul(") {
            sum += mul(pos, &line);
        }
    }
    println!("Sum of mul() instructions = {}", sum);
    // Part two: Only mul() in the do() sections count
    sum = 0;
    let line = lines
        .iter()
        .map(|s| s.chars())
        .flatten()
        .collect::<String>();
    let pos_do = line
        .match_indices("do()")
        .map(|(pos, _)| pos)
        .collect::<Vec<usize>>();
    let pos_dont = line
        .match_indices("don't()")
        .map(|(pos, _)| pos)
        .collect::<Vec<usize>>();
    // Lambda to check if we are in an enabled region
    let check_pos = move |pos_test: usize| -> bool {
        let p_do = pos_do
            .iter()
            .filter(|p| **p < pos_test)
            .max()
            .cloned()
            .unwrap_or(0);
        let p_dont = pos_dont
            .iter()
            .filter(|p| **p < pos_test)
            .max()
            .cloned()
            .unwrap_or(0);
        // If the nearest position of do() is smaller than the position of dont()
        // the region is invalid and mul() instructions dont count
        if p_do < p_dont {
            return false;
        }
        true
    };
    for (pos, _) in line.match_indices("mul(") {
        if check_pos(pos) {
            sum += mul(pos, &line);
        }
    }
    println!("Sum of mul() instructions in do() regions = {}", sum);
}

fn four(filename: &String) {
    let file = File::open(filename.as_str()).unwrap();
    let reader = BufReader::new(file);
    // Part one: Contruct a matrix of characters
    let mat = reader
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    // Lambda for determining the next position in a specific direction
    let next_pos =
        move |i: usize, j: usize, dir: usize, mat: &Vec<Vec<char>>| -> Option<(usize, usize)> {
            if i >= mat.len() || j >= mat[i].len() {
                return None;
            }
            // Convert to isize for comparing values smaller than zero
            let (i_loc, j_loc) = (i as isize, j as isize);
            let pos_new = match dir {
                0 => Some((i_loc + 1, j_loc)),
                1 => Some((i_loc, j_loc - 1)),
                2 => Some((i_loc - 1, j_loc)),
                3 => Some((i_loc, j_loc + 1)),
                4 => Some((i_loc + 1, j_loc + 1)),
                5 => Some((i_loc + 1, j_loc - 1)),
                6 => Some((i_loc - 1, j_loc - 1)),
                7 => Some((i_loc - 1, j_loc + 1)),
                _ => None,
            };
            // Check the new position against boundaries
            match pos_new {
                Some((ii, jj)) => {
                    if ii < 0 || jj < 0 {
                        return None;
                    }
                    let (i_new, j_new) = (ii as usize, jj as usize);
                    if i_new >= mat.len() || j_new >= mat[ii as usize].len() {
                        return None;
                    }
                    Some((i_new, j_new))
                }
                _ => None,
            }
        };
    // Lambda that returns the number of XMAS combinations found at (i, j)
    let find_xmas = move |i: usize, j: usize, mat: &Vec<Vec<char>>| -> u64 {
        const N_DIRS: usize = 8;
        const SEARCH_STR: &str = "XMAS";
        if mat[i][j] != 'X' {
            return 0u64;
        }
        let mut sum = 0u64;
        for d in 0..N_DIRS {
            let (mut i_next, mut j_next) = (i, j);
            for (ind, c) in SEARCH_STR.chars().enumerate() {
                if mat[i_next][j_next] == c {
                    // If this is the last char, we found XMAS
                    if ind == SEARCH_STR.len() - 1 {
                        sum += 1;
                    } else {
                        match next_pos(i_next, j_next, d, &mat) {
                            Some((i_n, j_n)) => {
                                i_next = i_n;
                                j_next = j_n;
                            }
                            None => {
                                break;
                            }
                        }
                    }
                } else {
                    break;
                }
            }
        }
        sum
    };
    let n_rows = mat.len();
    let mut sum = 0u64;
    for i in 0..n_rows {
        let n_cols = mat[i].len();
        for j in 0..n_cols {
            sum += find_xmas(i, j, &mat);
        }
    }
    println!("XMAS occurrences sum = {}", sum);
    // Part two: Find two MAS forming an X
    let find_x_mas = move |i: usize, j: usize, mat: &Vec<Vec<char>>| -> bool {
        // We search for the A in the middle
        if mat[i][j] != 'A' {
            return false;
        }
        // We only need the diagonal directions
        let dirs = [4, 6, 5, 7];
        let mut test = [' '; 4];
        for d in 0..dirs.len() {
            let dir = dirs[d];
            match next_pos(i, j, dir, &mat) {
                Some((i_n, j_n)) => {
                    let c = mat[i_n][j_n];
                    if c == 'M' || c == 'S' {
                        test[d] = c;
                    } else {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
        if ((test[0] == 'M' && test[1] == 'S') || (test[0] == 'S' && test[1] == 'M'))
            && ((test[2] == 'M' && test[3] == 'S') || (test[2] == 'S' && test[3] == 'M'))
        {
            return true;
        }
        false
    };
    sum = 0u64;
    for i in 0..n_rows {
        let n_cols = mat[i].len();
        for j in 0..n_cols {
            if find_x_mas(i, j, &mat) {
                sum += 1;
            }
        }
    }
    println!("X-MAS occurrences sum = {}", sum);
}

fn five(filename: &String) {
    let file = File::open(filename.as_str()).unwrap();
    let reader = BufReader::new(file);
    // Part one: Parse the rules and lines to check
    let mut rules = vec![];
    let mut lines_pages = vec![];
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.contains("|") {
            let rule = line
                .split("|")
                .map(|s| u64::from_str(s).unwrap())
                .collect::<Vec<u64>>();
            if rule.len() == 2 {
                rules.push((rule[0], rule[1]));
            }
        } else if line.contains(",") {
            let page_numbers = line
                .split(",")
                .map(|s| u64::from_str(s).unwrap())
                .collect::<Vec<u64>>();
            if page_numbers.len() > 0 {
                lines_pages.push(page_numbers);
            }
        }
    }
    // Returns the mid page number or 0, if the line is invalid
    let check_line = move |line: &Vec<u64>, rules: &Vec<(u64, u64)>| -> u64 {
        for (pos, page_number) in line.iter().enumerate() {
            // Find every rule for this number
            for (_, page_after) in rules.iter().filter(|(num, _)| num == page_number) {
                // Check if the page, which should come after the current
                // page number, is present before
                match line[0..pos].iter().find(|&page| page == page_after) {
                    Some(_) => return 0u64,
                    None => {}
                }
            }
        }
        let mid = line.len() / 2;
        line[mid]
    };
    let mut sum = 0u64;
    for line in lines_pages.iter() {
        sum += check_line(line, &rules);
    }
    println!("Sum of mid pages = {}", sum);
    // Part two: Correct the invalid lines
    let correct_invalid = move |line: &Vec<u64>, rules: &Vec<(u64, u64)>| -> u64 {
        let mut line_corr = line.clone();
        let mut valid = true;
        // Save the positions we need to swap
        let mut swap_pos = None;
        loop {
            for (pos, page_number) in line_corr.iter().enumerate() {
                // Find every rule for this number
                for (_, page_after) in rules.iter().filter(|(num, _)| num == page_number) {
                    // Check if the page, which should come after the current
                    // page number, is present before
                    match line_corr[0..pos]
                        .iter()
                        .enumerate()
                        .find(|(_, page)| *page == page_after)
                    {
                        Some((pos_invalid, _)) => {
                            swap_pos = Some((pos, pos_invalid));
                            break;
                        }
                        None => {}
                    }
                }
                match swap_pos {
                    Some((_, _)) => {
                        break;
                    }
                    None => {}
                }
            }
            match swap_pos {
                Some((pos, pos_invalid)) => {
                    valid = false;
                    line_corr.as_mut_slice().swap(pos, pos_invalid);
                    swap_pos = None;
                }
                None => {
                    break;
                }
            }
        }
        if !valid {
            let mid = line_corr.len() / 2;
            return line_corr[mid];
        }
        0u64
    };
    sum = 0u64;
    for line in lines_pages.iter() {
        sum += correct_invalid(line, &rules);
    }
    println!("Sum of mid pages corrected lines = {}", sum);
}

fn six(filename: &String) {
    #[derive(Debug, Copy, Clone, PartialEq)]
    enum Dir {
        Left,
        Right,
        Up,
        Down,
    }
    let file = File::open(filename.as_str()).unwrap();
    let reader = BufReader::new(file);
    let mut map = vec![];
    let mut pos_start = (0usize, 0usize);
    let mut dir_start = Dir::Up;
    // Part one: Parse the map and find out the initial direction and position
    for (row, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        map.push(vec![]);
        for (col, c) in line.chars().enumerate() {
            let field = match c {
                '.' => Some(c),
                '#' => Some(c),
                '^' => {
                    pos_start = (row, col);
                    dir_start = Dir::Up;
                    Some('.')
                }
                'v' => {
                    pos_start = (row, col);
                    dir_start = Dir::Down;
                    Some('.')
                }
                '<' => {
                    pos_start = (row, col);
                    dir_start = Dir::Left;
                    Some('.')
                }
                '>' => {
                    pos_start = (row, col);
                    dir_start = Dir::Right;
                    Some('.')
                }
                _ => None,
            };
            match field {
                Some(f) => {
                    map[row].push(f);
                }
                None => {}
            }
        }
    }
    // Lambda for determining the next direction after turning right
    let next_dir = move |dir: Dir| -> Dir {
        match dir {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    };
    // Lambda which finds the next position before an obstacle
    let next_pos =
        move |pos: (usize, usize), dir: Dir, map: &Vec<Vec<char>>| -> Option<(usize, usize)> {
            let (row, col) = pos;
            match dir {
                Dir::Up => {
                    if row > 0 {
                        return Some((row - 1, col));
                    }
                    None
                }
                Dir::Down => {
                    if row < (map.len() - 1) {
                        return Some((row + 1, col));
                    }
                    None
                }
                Dir::Left => {
                    if col > 0 {
                        return Some((row, col - 1));
                    }
                    None
                }
                Dir::Right => {
                    if col < (map[row].len() - 1) {
                        return Some((row, col + 1));
                    }
                    None
                }
            }
        };
    // Lambda for checking if there is an obstacle in front
    let check_obstacle_before =
        move |pos: (usize, usize), dir: Dir, map: &Vec<Vec<char>>| -> bool {
            match next_pos(pos, dir, map) {
                Some((row, col)) => {
                    if map[row][col] == '#' {
                        return true;
                    }
                    false
                }
                _ => false,
            }
        };
    // Lambda for counting positions on the map marked with X
    let count_fields = move |map: &Vec<Vec<char>>| -> u64 {
        let mut sum = 0u64;
        for r in map.iter() {
            for c in r.iter() {
                if *c == 'X' {
                    sum += 1;
                }
            }
        }
        sum
    };
    let mut pos = pos_start;
    let mut dir = dir_start;
    loop {
        if check_obstacle_before(pos, dir, &map) {
            dir = next_dir(dir);
        }
        let (row, col) = pos;
        map[row][col] = 'X';
        match next_pos(pos, dir, &map) {
            Some(pos_next) => {
                pos = pos_next;
            }
            None => {
                let sum = count_fields(&map);
                println!("Number of fields passed = {}", sum);
                break;
            }
        }
    }
    // Part two: Count number of possible positions for looping the guard
    let reset_fields = move |map: &mut Vec<Vec<char>>| {
        for r in map.iter_mut() {
            for c in r.iter_mut() {
                if *c == 'X' {
                    *c = '.';
                }
            }
        }
    };
    // Store the positions of the path
    let path = {
        let mut ret = vec![];
        for (row, r) in map.iter().enumerate() {
            for (col, c) in r.iter().enumerate() {
                // Don't place an obstacle at the starting position
                let pos_path = (row, col);
                if *c == 'X' && pos_path != pos_start {
                    ret.push((row, col));
                }
            }
        }
        ret
    };
    let mut sum = 0u64;
    reset_fields(&mut map);
    let max_steps = map.len() * map[0].len() * 4;
    for &pos_new_obstacle in path.iter() {
        // Mark the new obstacle on the map
        let (row_new_obst, col_new_obst) = pos_new_obstacle;
        map[row_new_obst][col_new_obst] = '#';
        pos = pos_start;
        dir = dir_start;
        let mut n_steps = 0usize;
        loop {
            // Check if the patrol got stuck in a loop
            if n_steps > max_steps {
                sum += 1;
                map[row_new_obst][col_new_obst] = '.';
                break;
            }
            // Turn until way is not blocked by an obstacle anymore
            let mut n_turns = 0;
            while check_obstacle_before(pos, dir, &map) && n_turns < 4 {
                dir = next_dir(dir);
                n_turns += 1;
            }
            match next_pos(pos, dir, &map) {
                Some(pos_next) => {
                    pos = pos_next;
                    n_steps += 1;
                }
                None => {
                    map[row_new_obst][col_new_obst] = '.';
                    break;
                }
            }
        }
    }
    println!("Number of possible loops = {}", sum);
}

fn seven(filename: &String) {
    let file = File::open(filename.as_str()).unwrap();
    let reader = BufReader::new(file);
    // Part one: Read the equations and check if they are valid with + and * operators
    let mut eqs = vec![];
    for line in reader.lines().map(|l| l.unwrap()) {
        let s = line.split(':').collect::<Vec<&str>>();
        if s.len() == 2 {
            let res = u64::from_str(s[0]).unwrap();
            let numbers = s[1].split(' ').filter(|x| x.len() > 0)
                .map(|x| u64::from_str(x).unwrap())
                .collect::<Vec<u64>>();
            eqs.push((res, numbers));
        }
    }
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum Op {
        Add,
        Mul,
        Concat,
    }
    impl Op {
        // Translates a byte into an enum
        pub fn from_u8(ind: u8) -> Self {
            match ind {
                0 => Op::Add,
                1 => Op::Mul,
                2 => Op::Concat,
                _ => Op::Add
            }
        }
    }
    // Lambda for evaluating an equation
    let eval = move |numbers: &Vec<u64>, counters: &Vec<u8>| -> u64 {
        let mut ret = numbers[0];
        for (ind, &c) in counters.iter().enumerate() {
            match Op::from_u8(c) {
                Op::Add => { ret += numbers[ind + 1]; },
                Op::Mul => { ret *= numbers[ind + 1]; },
                // Added for part two
                Op::Concat => { 
                    let num_str = numbers[ind + 1].to_string();
                    let ret_str = ret.to_string() + &num_str;
                    ret = u64::from_str(&ret_str).unwrap();
                }
            }
        }
        ret
    };
    // Lambda for validating an equation with any combination of operators
    let check_eq = move |res: u64, numbers: &Vec<u64>, n_ops: u8| -> bool {
        if numbers.len() == 0 {
            return false;
        }
        if numbers.len() == 1 {
            return res == numbers[0];
        }
        // Iterate through all possible combinations
        let mut counters = vec![0u8; numbers.len() - 1];
        while counters[counters.len() - 1] < n_ops {
            if eval(&numbers, &counters) == res {
                return true;
            }
            counters[0] += 1;
            for c in 0..(counters.len() - 1) {
                if counters[c] >= n_ops {
                    counters[c] = 0;
                    counters[c + 1] += 1;
                }
            }
        }
        false
    };
    // Compute the sum of all valid equation results
    let mut sum = 0u64;
    for (res, numbers) in eqs.iter() {
        if check_eq(*res, numbers, 2) {
            sum += res;
        }
    }
    println!("Sum of valid equations results = {:?}", sum);
    // Part two: Concatenation operator
    // Compute the sum of all valid equation results
    let mut sum = 0u64;
    for (res, numbers) in eqs.iter() {
        if check_eq(*res, numbers, 3) {
            sum += res;
        }
    }
    println!("Sum of valid equations with concat operator || results = {:?}", sum);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        let arg = args[1].as_str();
        let day = u64::from_str(arg).unwrap();
        let filename = &args[2];
        match day {
            1 => {
                one(filename);
            }
            2 => {
                two(filename);
            }
            3 => {
                three(filename);
            }
            4 => {
                four(filename);
            }
            5 => {
                five(filename);
            }
            6 => {
                six(filename);
            }
            7 => {
                seven(filename);
            }
            _ => println!("Unknown day {}", day),
        }
    } else {
        println!("At least two arguments required");
    }
}
