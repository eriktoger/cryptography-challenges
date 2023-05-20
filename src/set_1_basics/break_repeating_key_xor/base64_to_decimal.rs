use base64::{engine::general_purpose, Engine as _};

pub fn base64_to_decimal(base64_string: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(base64_string).unwrap()
}

#[cfg(test)]
mod tests_base64_to_decimal {
    use super::*;

    #[test]
    fn test_case_1() {
        let answer = base64_to_decimal("HUIf");
        assert_eq!(answer, vec![29, 66, 31]);
    }
}
