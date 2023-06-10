use super::BigInt;

impl BigInt {
    pub fn to_string(&self) -> String {
        let s: String = self.value.iter().map(|x| (x + 48) as u8 as char).collect();
        if self.is_negative {
            format!("-{s}")
        } else {
            s
        }
    }

    //we_should have from hex_string from base64_string, from_binary_string?
    pub fn from_str(s: &str) -> BigInt {
        let mut bi = BigInt {
            value: vec![],
            is_negative: false,
            rest: vec![0],
        };

        //should check that it is only integers, and no leading zeros
        for (i, c) in s.chars().enumerate() {
            if i == 0 && c == '-' {
                bi.is_negative = true;
                continue;
            }
            bi.value.push(((c as u8) - 48).into());
        }

        bi.add(&BigInt {
            value: vec![0],
            is_negative: false,
            rest: vec![0],
        })
    }
    pub fn from_int<T: std::fmt::Display>(integer: T) -> Option<BigInt> {
        let s = integer.to_string();
        if s.is_empty() {
            return None;
        }
        let mut is_negative = false;
        let mut value = vec![];
        for (i, c) in s.chars().enumerate() {
            if i == 0 && c == '-' {
                is_negative = true;
                continue;
            }
            let x = (c as i32) - 48;
            if x < 0 || x > 9 {
                return None;
            }
            value.push(x);
        }
        Some(BigInt {
            value,
            is_negative,
            rest: vec![0],
        })
    }
}

#[cfg(test)]
mod tests_big_int_multiply {
    use super::*;
    #[test]
    fn test_case_to_string() {
        let bi = BigInt::new();
        assert_eq!(bi.to_string(), "0");
    }

    #[test]
    fn test_case_from_str() {
        let bi = BigInt::from_str("123");
        assert_eq!(bi.to_string(), "123");
    }

    #[test]
    fn test_case_from_int_1() {
        let bi_1 = BigInt::from_int(123);
        assert_eq!(bi_1.is_some(), true);
        let bi = bi_1.expect("");
        assert_eq!(bi.to_string(), "123")
    }
    #[test]
    fn test_case_from_int_2() {
        let bi_1 = BigInt::from_int(-123);
        assert_eq!(bi_1.is_some(), true);
        let bi = bi_1.expect("");
        assert_eq!(bi.to_string(), "-123")
    }

    #[test]
    fn test_case_from_int_3() {
        let bi_1 = BigInt::from_int(132.3);
        assert_eq!(bi_1.is_none(), true);
    }
    #[test]
    fn test_case_from_int_4() {
        let bi_1 = BigInt::from_str("06");
        assert_eq!(bi_1.to_string(), "6");
    }
}
