use std::fs;
use std::collections::HashMap;

fn get_data() -> Vec<String> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/day2_input.txt");
    let contents = fs::read_to_string(path).expect("Can't read file");
    let lines = contents.lines().filter(|line| !line.is_empty()).map(|x| x.to_owned()).collect();
    lines
}

pub fn solve_part1() -> i32 {
    let lines = get_data();
    let mut mappings: HashMap<String, i32> = HashMap::new();
    mappings.insert(String::from("B X"), 1);
    mappings.insert(String::from("C Y"), 2);
    mappings.insert(String::from("A Z"), 3);
    mappings.insert(String::from("A X"), 4);
    mappings.insert(String::from("B Y"), 5);
    mappings.insert(String::from("C Z"), 6);
    mappings.insert(String::from("C X"), 7);
    mappings.insert(String::from("A Y"), 8);
    mappings.insert(String::from("B Z"), 9);
    
    let mut score: i32 = 0;
    for line in &lines {
        score += mappings.get(line).unwrap();
    }
    score
}

pub fn solve_part2() -> i32 {
    let lines = get_data();
    let mut mappings: HashMap<String, i32> = HashMap::new();
    mappings.insert(String::from("B X"), 1);
    mappings.insert(String::from("C X"), 2);
    mappings.insert(String::from("A X"), 3);
    mappings.insert(String::from("A Y"), 4);
    mappings.insert(String::from("B Y"), 5);
    mappings.insert(String::from("C Y"), 6);
    mappings.insert(String::from("C Z"), 7);
    mappings.insert(String::from("A Z"), 8);
    mappings.insert(String::from("B Z"), 9);
    let mut score: i32 = 0;
    for line in &lines {
        score += mappings.get(line).unwrap();
    }
    score
}
