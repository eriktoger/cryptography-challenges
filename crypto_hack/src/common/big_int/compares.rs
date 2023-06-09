use super::BigInt;

impl BigInt {
    pub fn greater_than(&self, other: &BigInt) -> bool {
        if self.is_negative && !other.is_negative {
            return false;
        }

        if !self.is_negative && other.is_negative {
            return true;
        }

        let self_len = self.value.len();
        let other_len = other.value.len();

        if !self.is_negative && !other.is_negative {
            if self_len > other_len {
                return true;
            }
            if self_len < other_len {
                return false;
            }
            return self.to_string() > other.to_string();
        }

        if self_len > other_len {
            return false;
        }
        if self_len < other_len {
            return true;
        }
        return self.to_string() < other.to_string();
    }

    pub fn is_odd(&self) -> bool {
        let last = self.value.last();
        match last {
            Some(x) => x % 2 == 1,
            None => false,
        }
    }
    pub fn is_even(&self) -> bool {
        !self.is_odd()
    }
}

#[cfg(test)]
mod tests_big_int_compares {
    use super::*;
    #[test]
    fn test_case_compares_1() {
        let bi_1 = BigInt::from_int(25).expect("");
        let bi_2 = BigInt::from_int(5).expect("");
        let answer = bi_1.greater_than(&bi_2);
        assert_eq!(answer, true);
    }
    #[test]
    fn test_case_compares_2() {
        let bi_1 = BigInt::from_int(5).expect("");
        let bi_2 = BigInt::from_int(6).expect("");
        let answer = bi_1.greater_than(&bi_2);
        assert_eq!(answer, false);
    }
    #[test]
    fn test_case_compares_3() {
        let bi_1 = BigInt::from_int(-25).expect("");
        let bi_2 = BigInt::from_int(5).expect("");
        let answer = bi_1.greater_than(&bi_2);
        assert_eq!(answer, false);
    }
    #[test]
    fn test_case_compares_4() {
        let bi_1 = BigInt::from_int(5).expect("");
        let bi_2 = BigInt::from_int(5).expect("");
        let answer = bi_1.greater_than(&bi_2);
        assert_eq!(answer, false);
    }
    #[test]
    fn test_case_compares_5() {
        let bi_1 = BigInt::from_int(-5).expect("");
        let bi_2 = BigInt::from_int(-6).expect("");
        let answer = bi_1.greater_than(&bi_2);
        assert_eq!(answer, true);
    }
    #[test]
    fn test_case_compares_6() {
        let bi_1 = BigInt::from_int(15).expect("");
        assert_eq!(bi_1.is_odd(), true);
    }
    #[test]
    fn test_case_compares_7() {
        let bi_1 = BigInt::from_int(14).expect("");
        assert_eq!(bi_1.is_odd(), false);
    }
    #[test]
    fn test_case_compares_8() {
        let bi_1 = BigInt::from_int(14).expect("");
        assert_eq!(bi_1.is_even(), true);
    }
    #[test]
    fn test_case_compares_9() {
        let bi_1 = BigInt::from_int(15).expect("");
        assert_eq!(bi_1.is_even(), false);
    }
}
