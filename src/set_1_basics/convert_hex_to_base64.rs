use base64::{engine::general_purpose, Engine as _};

pub fn convert_hex_to_base64(hex: &str) -> String {
    let data: Vec<u8> = hex
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|byte| u8::from_str_radix(&byte.iter().collect::<String>(), 16).unwrap())
        .collect();
    general_purpose::STANDARD.encode(&data)
}

#[cfg(test)]
mod tests_convert_hex_to_base64 {
    use super::*;
    #[test]
    fn test_case_1() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = convert_hex_to_base64(hex);
        assert_eq!(
            base64,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
