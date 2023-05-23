use super::{decrypt_text, get_cipher};
use crate::set_1_basics::get_lines;

pub fn break_repeating_key_xor() -> String {
    let all_lines = get_lines("src/set_1_basics/6.txt");
    let cipher = get_cipher(&all_lines);
    decrypt_text(&all_lines, cipher)
}

#[cfg(test)]
mod tests_break_repeating_key_xor {
    use super::*;

    #[test]
    fn test_case_1() {
        let answer = break_repeating_key_xor();
        assert_eq!(
            answer.starts_with("I'm back and I'm ringin' the bell "),
            true
        );
    }
}
