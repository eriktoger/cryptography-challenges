pub fn single_byte_xor_ciphers(bytes: Vec<u8>) -> char {
    let mut max_count = 0;
    let mut best_char = '-';

    for c in 0..255 {
        if c == 84 {
            println!();
        }
        let xor_bytes: Vec<u8> = bytes.iter().map(|b| (b ^ c)).collect();
        let count = xor_bytes
            .iter()
            .filter(|u| (**u > 64 && **u < 123) || **u == 32)
            .count();

        if count > max_count {
            max_count = count;
            best_char = c as char;
        }
    }

    best_char
}

#[cfg(test)]
mod tests_single_byte_xor_ciphers {
    use super::*;

    #[test]
    fn test_case_1() {
        let bytes =
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".as_bytes();

        let mut decimals: Vec<u8> = Vec::new();

        for i in (0..bytes.len()).step_by(2) {
            let hex_pair = std::str::from_utf8(&bytes[i..i + 2]).expect("Invalid UTF-8");
            let decimal = u8::from_str_radix(hex_pair, 16).expect("Invalid hexadecimal");
            decimals.push(decimal);
        }
        assert_eq!(single_byte_xor_ciphers(decimals), 'X');
    }
}
