pub fn single_byte_xor_ciphers(input: &str) -> String {
    let bytes: Vec<u8> = input
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|byte| u8::from_str_radix(&byte.iter().collect::<String>(), 16).unwrap())
        .collect();

    let mut max_count = 0;
    let mut best_char = '-';
    let mut best_match = "".to_string();

    for c in 0..255 {
        let xor_bytes: Vec<u8> = bytes.iter().map(|b| (b ^ c)).collect();
        let count = xor_bytes
            .iter()
            .filter(|u| (**u > 64 && **u < 123) || **u == 32)
            .count();

        if count > max_count {
            max_count = count;
            best_char = c as char;
            best_match = xor_bytes.iter().map(|c| *c as char).collect();
        }
    }

    format!("{best_char}-{max_count}-{best_match}")
}

#[cfg(test)]
mod tests_single_byte_xor_ciphers {
    use super::*;
    #[test]
    fn test_case_1() {
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        assert_eq!(
            single_byte_xor_ciphers(input),
            "X-33-Cooking MC's like a pound of bacon"
        );
    }
}
