use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_lines() -> String {
    let file = File::open("src/set_1_basics/6.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    lines.join("")
}
