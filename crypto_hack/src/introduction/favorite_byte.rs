use crate::common::hex_to_decimals;

pub fn favorite_byte() -> String {
    let decimals =
        hex_to_decimals("73626960647f6b206821204f21254f7d694f7624662065622127234f726927756d");
    for x in 0..=u8::MAX {
        let s: String = decimals.iter().map(|d| (*d ^ x) as char).collect();
        if s.find("crypto").is_some() {
            return s;
        }
    }
    String::new()
}

#[cfg(test)]
mod tests_favorite_byte {
    use super::*;
    #[test]
    fn test_case_1() {
        let answer = favorite_byte();
        let password = String::from("crypto{0x10_15_my_f4v0ur173_by7e}");
        assert_eq!(answer, password);
    }
}
