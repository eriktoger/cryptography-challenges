use aes::cipher::KeyInit;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt};
use aes::Aes128;

// inspired by: https://dev.to/nvn/cryptopals-crypto-challenges-using-rust-aes-in-ecb-mode-9bl
pub fn aes_in_ecb_mode(line_bytes: &Vec<u8>, key_str: &str) -> Vec<u8> {
    let mut blocks: Vec<GenericArray<u8, _>> = line_bytes
        .chunks(16)
        .map(|chunk| GenericArray::clone_from_slice(chunk))
        .collect();

    let key = GenericArray::clone_from_slice(key_str.as_bytes());
    let cipher = Aes128::new(&key);
    cipher.decrypt_blocks(&mut blocks);
    blocks.iter().flatten().copied().collect()
}

#[cfg(test)]
mod tests_aes_in_ecb_mode {
    use super::*;
    use crate::set_1_basics::{base64_to_decimal, get_lines};
    #[test]
    fn test_aes_in_ecb_mode() {
        let all_lines = get_lines("src/set_1_basics/7.txt");
        let line_bytes = base64_to_decimal(&all_lines);
        let bytes = aes_in_ecb_mode(&line_bytes, "YELLOW SUBMARINE");
        let text: String = bytes.iter().map(|x| *x as char).collect();
        assert_eq!(text.starts_with("I'm back and I'm ringin' the bell "), true);
    }
}
