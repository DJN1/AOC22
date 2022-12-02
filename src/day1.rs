use std::fs;

pub fn get_list_elfs() -> Vec<i32> {
    let contents = fs::read_to_string("day1_input.txt").expect("Can't read file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut elfs: Vec<i32> = Vec::new();
    let mut total = 0;
    for line in &lines {
        if !line.is_empty() {
            total += line.parse::<i32>().unwrap();
        } else {
            elfs.push(total);
            total = 0;
        }
    }
    elfs.sort();
    elfs
}

pub fn solve_part1() -> i32 {
    let elfs = get_list_elfs();
    elfs.last().unwrap().clone()
}

pub fn solve_part2() -> i32 {
    let elfs = get_list_elfs();
    elfs.iter().rev().take(3).sum()
}

