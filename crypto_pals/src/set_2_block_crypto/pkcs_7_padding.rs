pub fn pkcs_7_padding(s: &str, pad_to: usize) -> String {
    let mut new_s = s.to_string();
    while new_s.len() < pad_to {
        new_s.push('\x04');
    }
    new_s
}

#[cfg(test)]
mod tests_pkcs_7_padding {
    use super::*;
    #[test]
    fn test_case_1() {
        assert_eq!(
            pkcs_7_padding("YELLOW SUBMARINE", 20),
            "YELLOW SUBMARINE\x04\x04\x04\x04"
        );
    }
}
