fn hex_to_char(bytes: &[char]) -> char {
    u8::from_str_radix(&bytes.iter().collect::<String>(), 16).unwrap() as char
}

pub fn hex() -> String {
    let input = "63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
    input
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(hex_to_char)
        .collect()
}

#[cfg(test)]
mod tests_hex {
    use super::*;
    #[test]
    fn test_case_1() {
        let answer = hex();
        let password = String::from("crypto{You_will_be_working_with_hex_strings_a_lot}");
        assert_eq!(answer, password);
    }
}
