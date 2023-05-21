use hex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_lines_as_vec(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}
// inpsired by: https://dev.to/nvn/cryptopals-crypto-challenges-using-rust-detect-aes-in-ecb-mode-481n
pub fn detect_aes_ecb_encryption(path: &str) -> usize {
    let mut best_line: usize = 0;
    let mut best_identical_blocks: usize = usize::MAX;

    let lines = get_lines_as_vec(path);

    for (i, line) in lines.iter().enumerate() {
        let bytes = hex::decode(line).unwrap();
        let blocks: Vec<_> = bytes.chunks(16).collect();

        let unique_blocks = blocks.iter().collect::<HashSet<_>>().len();

        if unique_blocks < best_identical_blocks {
            best_identical_blocks = unique_blocks;
            best_line = i;
        }
    }

    best_line
}

#[cfg(test)]
mod tests_detect_aes_ecb_encryption {
    use super::*;
    #[test]
    fn test_case_1() {
        let line_index = detect_aes_ecb_encryption("src/set_1_basics/8.txt");
        assert_eq!(line_index, 132);
    }
}
