fn hex_to_decimal(bytes: &[char]) -> u8 {
    u8::from_str_radix(&bytes.iter().collect::<String>(), 16).unwrap()
}

pub fn hex_to_decimals(input: &str) -> Vec<u8> {
    input
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(hex_to_decimal)
        .collect()
}
