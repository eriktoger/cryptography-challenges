use base64::{engine::general_purpose, Engine as _};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_lines(path: &str) -> String {
    let file = File::open(path).unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    lines.join("")
}

pub fn base64_to_decimal(base64_string: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(base64_string).unwrap()
}

#[cfg(test)]
mod tests_base64_to_decimal {
    use super::*;

    #[test]
    fn test_case_1() {
        let answer = base64_to_decimal("HUIf");
        assert_eq!(answer, vec![29, 66, 31]);
    }
}
