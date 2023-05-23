use base64::{engine::general_purpose, Engine as _};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::set_1_basics::aes_in_ecb_mode;

fn base64_to_decimal(base64_string: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(base64_string).unwrap()
}

fn get_lines_as_vec(path: &str) -> Vec<Vec<u8>> {
    let file = File::open(path).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| base64_to_decimal(&line.unwrap()))
        .collect()
}

pub fn cbc_mode() -> String {
    let bytes: Vec<u8> = get_lines_as_vec("src/set_2_block_crypto/10.txt")
        .iter()
        .flatten()
        .cloned()
        .collect();

    let key = "YELLOW SUBMARINE";
    let mut plain_text = "".to_string();
    let mut prev_ct: Vec<u8> = vec![0; 16];

    for chunk in bytes.chunks(16) {
        let chunk_vec = chunk.to_vec();
        aes_in_ecb_mode(&chunk_vec, key)
            .iter()
            .zip(prev_ct.iter())
            .map(|(a, b)| a ^ b)
            .for_each(|x| plain_text.push(x as char));

        prev_ct = chunk_vec;
    }

    plain_text
}

#[cfg(test)]
mod tests_cbc_mode {
    use super::*;
    #[test]
    fn test_case_1() {
        let plain_text = cbc_mode();
        let first_two_lines =
            "I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \n";
        assert_eq!(plain_text.starts_with(first_two_lines), true);
    }
}
