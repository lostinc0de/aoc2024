use std::collections::{HashMap, HashSet};
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
            let numbers = s[1]
                .split(' ')
                .filter(|x| x.len() > 0)
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
                _ => Op::Add,
            }
        }
    }
    // Lambda for evaluating an equation
    let eval = move |numbers: &Vec<u64>, counters: &Vec<u8>| -> u64 {
        let mut ret = numbers[0];
        for (ind, &c) in counters.iter().enumerate() {
            match Op::from_u8(c) {
                Op::Add => {
                    ret += numbers[ind + 1];
                }
                Op::Mul => {
                    ret *= numbers[ind + 1];
                }
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
    println!(
        "Sum of valid equations with concat operator || results = {:?}",
        sum
    );
}

fn eight(filename: &String) {
    let file = File::open(filename.as_str()).unwrap();
    let reader = BufReader::new(file);
    // Part one: Create map of antennas and find all anti nodes
    let mut map = vec![];
    for line in reader.lines().map(|l| l.unwrap()) {
        let row = line
            .chars()
            .filter(|&c| c == '.' || c.is_alphanumeric())
            .collect::<Vec<char>>();
        map.push(row);
    }
    // Store the positions in a hash map
    let mut positions = HashMap::<char, Vec<(isize, isize)>>::new();
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c.is_alphanumeric() {
                match positions.get_mut(c) {
                    Some(entries) => {
                        entries.push((i as isize, j as isize));
                    }
                    None => {
                        positions.insert(*c, vec![(i as isize, j as isize)]);
                    }
                }
            }
        }
    }
    // Lambda to check if a node position is valid or out of bounds
    let valid_node = move |pos: (isize, isize), bounds: (isize, isize)| -> bool {
        let (n_rows, n_cols) = bounds;
        let (row, col) = pos;
        if row >= 0 && row < n_rows && col >= 0 && col < n_cols {
            return true;
        }
        false
    };
    let bounds = (map.len() as isize, map[0].len() as isize);
    let mut positions_antinode = HashSet::<(isize, isize)>::new();
    for freq in positions.keys() {
        let ant = &positions[freq];
        for i in 0..ant.len() {
            let (row_i, col_i) = ant[i];
            for j in (i + 1)..ant.len() {
                let (row_j, col_j) = ant[j];
                // Compute the direction vector from node i to j
                let (row_dir, col_dir) = (row_j - row_i, col_j - col_i);
                // Add the direction vector to node j
                let node = (row_j + row_dir, col_j + col_dir);
                if valid_node(node, bounds) {
                    positions_antinode.insert(node);
                }
                // Subtract the direction vector from node i
                let node = (row_i - row_dir, col_i - col_dir);
                if valid_node(node, bounds) {
                    positions_antinode.insert(node);
                }
            }
        }
    }
    println!(
        "Number of distinct antinodes = {}",
        positions_antinode.len()
    );
    // Part two: Take harmonics into account
    let mut positions_antinode = HashSet::<(isize, isize)>::new();
    for freq in positions.keys() {
        let ant = &positions[freq];
        for i in 0..ant.len() {
            let (row_i, col_i) = ant[i];
            for j in (i + 1)..ant.len() {
                let (row_j, col_j) = ant[j];
                // Compute the direction vector from node i to j
                let (row_dir, col_dir) = (row_j - row_i, col_j - col_i);
                // Add the direction vector to node j
                let mut node = (row_j + row_dir, col_j + col_dir);
                while valid_node(node, bounds) {
                    positions_antinode.insert(node);
                    node = (node.0 + row_dir, node.1 + col_dir);
                }
                // Subtract the direction vector from node i
                let mut node = (row_i - row_dir, col_i - col_dir);
                while valid_node(node, bounds) {
                    positions_antinode.insert(node);
                    node = (node.0 - row_dir, node.1 - col_dir);
                }
                // Insert positions of antennae as well
                positions_antinode.insert(ant[i]);
                positions_antinode.insert(ant[j]);
            }
        }
    }
    println!(
        "Number of distinct antinodes with harmonics = {}",
        positions_antinode.len()
    );
}

fn nine(filename: &String) {
    let mut file = File::open(filename.as_str()).unwrap();
    // Part one: Read fragmented file system structure
    let mut disk_map = String::new();
    file.read_to_string(&mut disk_map).unwrap();
    // First byte is the number of blocks of the current file
    // and the second byte the number of free blocks to the next file
    const EMPTY: i32 = -1;
    let decode = move |disk: &String| -> Vec<i32> {
        let mut map_dec = vec![];
        // Decode the disk map
        let map_chunks = disk.as_bytes().chunks_exact(2);
        for (id, chunk) in map_chunks.enumerate() {
            let id_file = id as i32;
            let (b_file, b_free) = (chunk[0], chunk[1]);
            let n_blocks_file = b_file - b'0';
            if n_blocks_file > 0 && n_blocks_file <= 9 {
                let mut block = vec![id_file; n_blocks_file as usize];
                map_dec.append(&mut block);
            }
            let n_blocks_free = b_free - b'0';
            if n_blocks_free > 0 && n_blocks_free <= 9 {
                let mut block = vec![EMPTY; n_blocks_free as usize];
                map_dec.append(&mut block);
            }
        }
        map_dec
    };
    // Defragment the map
    let defrag = move |map: &mut Vec<i32>| {
        let mut pos_file = map.len() - 1;
        let mut pos_free = 0;
        loop {
            while pos_file > 0 && map[pos_file] == EMPTY {
                pos_file -= 1;
            }
            while pos_free < map.len() && map[pos_free] != EMPTY {
                pos_free += 1;
            }
            if pos_file <= pos_free {
                break;
            }
            map.swap(pos_file, pos_free);
        }
    };
    // Lambda for computing the checksum
    let chksum = move |map: &Vec<i32>| -> u64 {
        let mut sum = 0;
        for (pos, &id) in map.iter().enumerate() {
            if id != EMPTY {
                sum += (pos as u64) * id as u64;
            }
        }
        sum
    };
    let mut map_dec = decode(&disk_map);
    defrag(&mut map_dec);
    println!("Check sum = {}", chksum(&map_dec));
    // Part two: Shift files to a suitable place on the left side
    // Lambda for finding a free position
    let find_free_pos = move |map: &Vec<i32>, len: usize| -> Option<usize> {
        let mut len_free = 0;
        for (pos, &val) in map.iter().enumerate() {
            if val == EMPTY {
                len_free += 1;
            } else {
                len_free = 0;
            }
            if len_free >= len {
                //println!("pos {} len free {}", pos, len_free);
                return Some(pos + 1 - len_free);
            }
        }
        None
    };
    // Lambda for shifting the file blocks to the left
    let shift_files = move |map: &mut Vec<i32>| {
        let mut pos_file = map.len() - 1;
        loop {
            // Skip empty blocks
            while pos_file > 0 && map[pos_file] == EMPTY {
                pos_file -= 1;
            }
            // Abort if we are on the left side
            if pos_file == 0 {
                break;
            }
            // Get the length of the current file block
            let id_file = map[pos_file];
            let mut len = 0;
            while pos_file > 0 && map[pos_file] == id_file {
                pos_file -= 1;
                len += 1;
            }
            // Find a free position for the file
            if let Some(pos_free) = find_free_pos(&map, len) {
                // The actual starting block of the file is plus one
                pos_file += 1;
                if pos_free < pos_file {
                    for i in 0..len {
                        map.swap(pos_free + i, pos_file + i);
                    }
                }
            }
            pos_file -= 1;
        }
    };
    let mut map_dec = decode(&disk_map);
    shift_files(&mut map_dec);
    println!("Check sum with preprocessing = {}", chksum(&map_dec));
}

fn ten(filename: &String) {
    let file = File::open(filename.as_str()).unwrap();
    let reader = BufReader::new(file);
    let mut map = vec![];
    for line in reader.lines().map(|l| l.unwrap()) {
        map.push(
            line.chars()
                .map(|c| {
                    if c.is_numeric() {
                        c as u8 - b'0'
                    } else {
                        255u8
                    }
                })
                .collect::<Vec<u8>>(),
        );
    }
    #[derive(Debug, Copy, Clone, PartialEq)]
    enum Dir {
        Left,
        Right,
        Up,
        Down,
    }
    // Find all trailheads
    let mut trailheads = vec![];
    for (row, r) in map.iter().enumerate() {
        for (col, b) in r.iter().enumerate() {
            if *b == 0 {
                trailheads.push((row, col));
            }
        }
    }
    // Lambda for determining the next position on the map in direction dir
    let next_pos =
        move |map: &Vec<Vec<u8>>, pos: (usize, usize), dir: Dir| -> Option<(usize, usize)> {
            let (n_rows, n_cols) = (map.len(), map[0].len());
            let (row, col) = pos;
            match dir {
                Dir::Left => {
                    if col > 0 {
                        return Some((row, col - 1));
                    }
                }
                Dir::Right => {
                    if col < (n_cols - 1) {
                        return Some((row, col + 1));
                    }
                }
                Dir::Up => {
                    if row > 0 {
                        return Some((row - 1, col));
                    }
                }
                Dir::Down => {
                    if row < (n_rows - 1) {
                        return Some((row + 1, col));
                    }
                }
            }
            None
        };
    // Lambda to check, if the next step is valid
    let valid_step =
        move |map: &Vec<Vec<u8>>, pos: (usize, usize), dir: Dir| -> Option<(usize, usize)> {
            let (row, col) = pos;
            let val = map[row][col];
            if let Some(pos_new) = next_pos(map, pos, dir) {
                let (row_new, col_new) = pos_new;
                let val_new = map[row_new][col_new];
                // Only ascending values
                if val_new == (val + 1) {
                    //println!(
                    //    "pos {:?} val {} pos_new {:?} val_new {}",
                    //    pos, val, pos_new, val_new
                    //);
                    return Some(pos_new);
                }
            }
            None
        };
    // Lambda for determining possible paths
    let find_paths = move |map: &Vec<Vec<u8>>, trailhead: (usize, usize)| -> (u64, u64) {
        const DIRS: [Dir; 4] = [Dir::Left, Dir::Right, Dir::Up, Dir::Down];
        let mut paths = vec![vec![trailhead]];
        // Only the individual reachable destinations count, not every path
        let mut reachable_dest = HashSet::<(usize, usize)>::new();
        // Part two: Count individual paths (accidentally done before)
        let mut n_valid_paths = 0u64;
        while let Some(mut path) = paths.pop() {
            loop {
                let pos = path[path.len() - 1];
                if path.len() == 10 {
                    reachable_dest.insert(pos);
                    n_valid_paths += 1;
                    break;
                }
                // Count the number of possible directions for the next move
                let mut count = 0;
                for &dir in DIRS.iter() {
                    if let Some(pos_new) = valid_step(&map, pos, dir) {
                        if count > 0 {
                            // We found a new path since the current path
                            // has a new position already
                            let mut path_new = path.clone();
                            // Exchange the last position
                            path_new[path.len() - 1] = pos_new;
                            paths.push(path_new);
                        } else {
                            // Process current path
                            path.push(pos_new);
                        }
                        count += 1;
                    }
                }
                // Abort if no possible path in any direction could be found
                if count == 0 {
                    break;
                }
            }
        }
        (reachable_dest.len() as u64, n_valid_paths)
    };
    let mut sum = 0;
    for &trailhead in trailheads.iter() {
        let (n_reachable_dest, _) = find_paths(&map, trailhead);
        sum += n_reachable_dest;
    }
    println!("Sum of hiking paths = {}", sum);
    // Part two: Count individual paths
    let mut sum = 0;
    for &trailhead in trailheads.iter() {
        let (_, n_paths_valid) = find_paths(&map, trailhead);
        sum += n_paths_valid;
    }
    println!("Sum of individual hiking paths = {}", sum);
}

fn eleven(filename: &String) {
    let file = File::open(filename.as_str()).unwrap();
    let reader = BufReader::new(file);
    let mut stones = vec![];
    for line in reader.lines().map(|l| l.unwrap()) {
        let stones_line = line.split_whitespace().map(|x| u64::from_str(x).unwrap());
        stones.extend(stones_line);
    }
    // Lambda for one blink splitting the stones
    let blink = move |stones: &mut Vec<u64>| {
        let mut stones_add = vec![];
        for stone in stones.iter_mut() {
            if *stone == 0 {
                *stone = 1;
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    // Split the stone
                    let half = stone_str.len() / 2;
                    let (s0, s1) = stone_str.split_at(half);
                    *stone = u64::from_str(s0).unwrap();
                    stones_add.push(u64::from_str(s1).unwrap());
                } else {
                    *stone *= 2024;
                }
            }
        }
        stones.extend(&stones_add);
    };
    // Part one: Blink 25 times
    let n_blinks = 25;
    for _i in 0..n_blinks {
        blink(&mut stones);
    }
    println!("Number of stones after 25 blinks {}", stones.len());
    // Part two: Blink 75 times
    // Use a hashmap since lots of values are recurring
    let blink_map = move |stones_map: &mut HashMap<u64, u64>| {
        let mut ret = HashMap::<u64, u64>::new();
        for (&stone, &count) in stones_map.iter() {
            if stone == 0 {
                ret.entry(1).and_modify(|c| *c += count).or_insert(count);
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    // Split the stone
                    let half = stone_str.len() / 2;
                    let (s0, s1) = stone_str.split_at(half);
                    let s0 = u64::from_str(s0).unwrap();
                    let s1 = u64::from_str(s1).unwrap();
                    ret.entry(s0).and_modify(|c| *c += count).or_insert(count);
                    ret.entry(s1).and_modify(|c| *c += count).or_insert(count);
                } else {
                    ret.entry(stone * 2024)
                        .and_modify(|c| *c += count)
                        .or_insert(count);
                }
            }
        }
        *stones_map = ret;
    };
    let mut stones_map = HashMap::<u64, u64>::new();
    for s in stones.iter() {
        stones_map.entry(*s).and_modify(|c| *c += 1).or_insert(1);
    }
    // Blink another 50 times
    let n_blinks = 50;
    for _i in 0..n_blinks {
        blink_map(&mut stones_map);
    }
    let sum = stones_map.values().sum::<u64>();
    println!("Number of stones after 75 blinks {}", sum);
}

fn twelve(filename: &String) {
    let file = File::open(filename.as_str()).unwrap();
    let reader = BufReader::new(file);
    let mut garden_map = vec![];
    for line in reader.lines().map(|l| l.unwrap()) {
        let garden_line = line.chars().collect::<Vec<char>>();
        garden_map.push(garden_line);
    }
    #[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
    enum Dir {
        Left,
        Right,
        Up,
        Down,
    }
    // Lambda for determining the next position in direction dir
    let next_pos =
        move |map: &Vec<Vec<char>>, pos: (usize, usize), dir: Dir| -> Option<(usize, usize)> {
            let (row, col) = pos;
            match dir {
                Dir::Left => {
                    if col > 0 {
                        return Some((row, col - 1));
                    }
                }
                Dir::Right => {
                    if col < (map[0].len() - 1) {
                        return Some((row, col + 1));
                    }
                }
                Dir::Up => {
                    if row > 0 {
                        return Some((row - 1, col));
                    }
                }
                Dir::Down => {
                    if row < (map.len() - 1) {
                        return Some((row + 1, col));
                    }
                }
            }
            None
        };
    // Lambda for checking, if neighboured field belongs to the same region
    let same_region =
        move |map: &Vec<Vec<char>>, pos: (usize, usize), dir: Dir| -> Option<(usize, usize)> {
            let (row, col) = pos;
            match next_pos(map, pos, dir) {
                Some((row_next, col_next)) => {
                    if map[row][col] == map[row_next][col_next] {
                        return Some((row_next, col_next));
                    }
                }
                _ => {}
            }
            None
        };
    // Lambda for computing the perimeter of a region
    let comp_perimeter = move |map: &Vec<Vec<char>>, fields: &Vec<(usize, usize)>| -> u64 {
        const DIRS: [Dir; 4] = [Dir::Left, Dir::Right, Dir::Up, Dir::Down];
        let mut sum_peri = 0;
        for &pos in fields.iter() {
            // The perimeter of a field equals 4 - the number of neighbours
            let mut n_neighbours = 0;
            for &dir in DIRS.iter() {
                if let Some(_) = same_region(map, pos, dir) {
                    n_neighbours += 1;
                }
            }
            sum_peri += 4 - n_neighbours;
        }
        sum_peri
    };
    // Store the region IDs for each position on the map
    let mut pos_region = vec![vec![None; garden_map[0].len()]; garden_map.len()];
    // Store the positions for each region in a vec
    let mut region_pos = vec![];
    const DIRS: [Dir; 4] = [Dir::Left, Dir::Right, Dir::Up, Dir::Down];
    for (row, r) in garden_map.iter().enumerate() {
        for (col, _) in r.iter().enumerate() {
            let pos = (row, col);
            // Check if current field already belongs to a region
            if pos_region[row][col] == None {
                // Add a new region
                let region_id = region_pos.len();
                pos_region[row][col] = Some(region_id);
                region_pos.push(vec![pos]);
                let mut neighbours = vec![pos];
                // Find all neighbour belonging to the same region
                while let Some(pos_next) = neighbours.pop() {
                    for &dir in DIRS.iter() {
                        if let Some(pos_neigh) = same_region(&garden_map, pos_next, dir) {
                            let (row_neigh, col_neigh) = pos_neigh;
                            if pos_region[row_neigh][col_neigh] == None {
                                let (row_neigh, col_neigh) = pos_neigh;
                                pos_region[row_neigh][col_neigh] = Some(region_id);
                                region_pos[region_id].push(pos_neigh);
                                neighbours.push(pos_neigh);
                            }
                        }
                    }
                }
            }
        }
    }
    // Finally compute the price
    let mut price = 0;
    for fields in region_pos.iter() {
        let peri = comp_perimeter(&garden_map, fields);
        let area = fields.len() as u64;
        price += peri * area;
    }
    println!("Price = {}", price);
    // Part two: Compute price using the number of sides instead of the area
    let mut price = 0;
    for fields in region_pos.iter() {
        let area = fields.len() as u64;
        let n_sides = {
            // Complementary directions
            const DIRS_COMPL: [[Dir; 2]; 4] = [
                [Dir::Up, Dir::Down],
                [Dir::Up, Dir::Down],
                [Dir::Left, Dir::Right],
                [Dir::Left, Dir::Right],
            ];
            // Store the side index for a field in a specific direction in a hash map
            let mut pos_sides = HashSet::<(Dir, (usize, usize))>::new();
            let mut n_sides = 0;
            for &pos in fields.iter() {
                for (ind, &dir) in DIRS.iter().enumerate() {
                    // Check if the field already belongs to a side in this direction
                    if pos_sides.get(&(dir, pos)) == None
                        && same_region(&garden_map, pos, dir) == None
                    {
                        n_sides += 1;
                        // Now check all the neighbours in the complementary directions
                        // if they belong to the same side
                        for &dir_compl in DIRS_COMPL[ind].iter() {
                            let mut pos_next = pos;
                            loop {
                                // Check if this field lies on a boundary
                                match same_region(&garden_map, pos_next, dir) {
                                    None => {
                                        // Mark the field as used in this direction
                                        pos_sides.insert((dir, pos_next));
                                    }
                                    _ => {
                                        break;
                                    }
                                }
                                // Check if there is a neighboured field in this region
                                match same_region(&garden_map, pos_next, dir_compl) {
                                    Some(pos_neigh) => {
                                        pos_next = pos_neigh;
                                    }
                                    None => {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            n_sides
        };
        price += n_sides * area;
    }
    println!("Price using sides = {}", price);
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
            8 => {
                eight(filename);
            }
            9 => {
                nine(filename);
            }
            10 => {
                ten(filename);
            }
            11 => {
                eleven(filename);
            }
            12 => {
                twelve(filename);
            }
            _ => println!("Unknown day {}", day),
        }
    } else {
        println!("At least two arguments required: Day and input filename");
    }
}
