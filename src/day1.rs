use std::fs;

pub fn get_list_elves() -> Vec<i32> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/day1_input.txt");
    let contents = fs::read_to_string(path).expect("Can't read file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut elves: Vec<i32> = Vec::new();
    let mut total = 0;
    for line in &lines {
        if !line.is_empty() {
            total += line.parse::<i32>().unwrap();
        } else {
            elves.push(total);
            total = 0;
        }
    }
    elves.sort();
    elves
}

pub fn solve_part1() -> i32 {
    let elves = get_list_elves();
    elves.last().unwrap().clone()
}

pub fn solve_part2() -> i32 {
    let elves = get_list_elves();
    elves.iter().rev().take(3).sum()
}

