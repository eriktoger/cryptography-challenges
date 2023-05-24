use base64::{engine::general_purpose, Engine as _};

pub fn bytes_to_base64(bytes: Vec<u8>) -> String {
    general_purpose::STANDARD.encode(bytes)
}

fn hex_to_ascci(bytes: &[char]) -> u8 {
    u8::from_str_radix(&bytes.iter().collect::<String>(), 16).unwrap()
}

pub fn base64_to_text() -> String {
    let input = "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf";
    let bytes: Vec<u8> = input
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(hex_to_ascci)
        .collect();

    bytes_to_base64(bytes)
}

#[cfg(test)]
mod tests_base64 {
    use super::*;
    #[test]
    fn test_case_1() {
        let answer = base64_to_text();
        let password = String::from("crypto/Base+64+Encoding+is+Web+Safe/");
        assert_eq!(answer, password);
    }
}
