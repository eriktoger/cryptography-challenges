use std::fs::File;
use std::io::{self, BufRead};

use crate::set_1_basics::single_byte_xor_chipers;

pub fn detect_single_character_xor() -> String {
    let file = File::open("src/set_1_basics/4.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut best_match = "".to_string();
    let mut best_count = 0;
    for line in lines {
        let chipered = single_byte_xor_chipers(&line.unwrap());
        let split_chiper: Vec<&str> = chipered.split("-").collect();
        let count = split_chiper[1].parse::<i32>().unwrap();
        if count > best_count {
            best_match = split_chiper[2].to_string();
            best_count = count;
        }
    }

    best_match
}

#[cfg(test)]
mod tests_detect_single_character_xor {
    use super::*;
    #[test]
    fn test_case_1() {
        let output = detect_single_character_xor();

        let answer = "Now that the party is jumping\n".to_string();
        assert_eq!(output, answer);
    }
}
