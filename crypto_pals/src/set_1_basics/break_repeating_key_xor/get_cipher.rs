use super::{find_keysize, single_byte_xor_ciphers};
use crate::set_1_basics::base64_to_decimal;

pub fn get_cipher(all_lines: &str) -> Vec<char> {
    let keysize = find_keysize(all_lines);

    let mut blocks = vec![vec![]; keysize];
    let mut current_index = 0;

    let line_bytes = base64_to_decimal(all_lines);
    for c in line_bytes {
        blocks[current_index].push(c);
        current_index = (current_index + 1) % keysize;
    }

    let mut cipher = vec![];
    for block in blocks {
        let ch = single_byte_xor_ciphers(block);
        cipher.push(ch);
    }
    cipher
}

#[cfg(test)]
mod tests_get_cipher {
    use super::*;
    use crate::set_1_basics::get_lines;

    #[test]
    fn test_case_1() {
        let all_lines = get_lines("src/set_1_basics/6.txt");
        let cipher = get_cipher(&all_lines);
        assert_eq!(
            cipher,
            vec![
                'T', 'e', 'r', 'm', 'i', 'n', 'a', 't', 'o', 'r', ' ', 'X', ':', ' ', 'B', 'r',
                'i', 'n', 'g', ' ', 't', 'h', 'e', ' ', 'n', 'o', 'i', 's', 'e'
            ]
        );
    }
}
