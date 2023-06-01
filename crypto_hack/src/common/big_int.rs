pub struct BigInt {
    value: Vec<i32>,
    is_negative: bool,
    rest: i32,
}

impl BigInt {
    fn new() -> BigInt {
        BigInt {
            value: vec![0],
            is_negative: false,
            rest: 0,
        }
    }
    fn to_string(&self) -> String {
        let s: String = self.value.iter().map(|x| (x + 48) as u8 as char).collect();
        if self.is_negative {
            format!("-{s}")
        } else {
            s
        }
    }

    //we_should have from hex_string from base64_string, from_binary_string?
    fn from_str(s: &str) -> BigInt {
        let mut bi = BigInt {
            value: vec![],
            is_negative: false,
            rest: 0,
        };

        //should check that it is only integers, and no leading zeros
        for (i, c) in s.chars().enumerate() {
            if i == 0 && c == '-' {
                bi.is_negative = true;
                continue;
            }
            bi.value.push(((c as u8) - 48).into());
        }
        bi
    }

    fn add(&self, bi: &BigInt) -> BigInt {
        if self.is_negative && bi.is_negative {
            let new_bi = BigInt {
                value: self.value.clone(),
                is_negative: false,
                rest: 0,
            };
            return bi.subtract(&new_bi);
        }

        if self.is_negative && !bi.is_negative {
            let new_bi = BigInt {
                value: self.value.clone(),
                is_negative: false,
                rest: 0,
            };
            return bi.subtract(&new_bi);
        }

        if !self.is_negative && bi.is_negative {
            let new_bi = BigInt {
                value: bi.value.clone(),
                is_negative: false,
                rest: 0,
            };
            return self.subtract(&new_bi);
        }

        let (longest, shortest) = if self.value.len() >= bi.value.len() {
            (&self.value, &bi.value)
        } else {
            (&bi.value, &self.value)
        };

        let longest_rev: Vec<&i32> = longest.iter().rev().collect();
        let mut shortest_rev: Vec<&i32> = shortest.iter().rev().collect();
        shortest_rev.resize(longest_rev.len(), &0);

        let mut rest = 0;
        let mut new_value: Vec<i32> = shortest_rev
            .iter()
            .zip(longest_rev)
            .map(|(a, b)| {
                let sum = *a + *b + rest;
                rest = sum / 10;

                return sum % 10;
            })
            .collect();

        if rest > 0 {
            new_value.push(rest);
        }

        new_value.reverse();
        return BigInt {
            value: new_value,
            is_negative: false,
            rest: 0,
        };
    }

    fn subtract(&self, bi: &BigInt) -> BigInt {
        if bi.is_negative {
            let new_bi = BigInt {
                value: bi.value.clone(),
                is_negative: false,
                rest: 0,
            };
            return self.add(&new_bi);
        }

        if self.is_negative {
            let new_bi = BigInt {
                value: self.value.clone(),
                is_negative: false,
                rest: 0,
            };

            return BigInt {
                value: new_bi.add(&bi).value,
                is_negative: true,
                rest: 0,
            };
        }

        let mut first: Vec<&i32> = self.value.iter().rev().collect();
        let mut second: Vec<&i32> = bi.value.iter().rev().collect();

        let length = first.len().max(second.len());
        first.resize(length, &0);
        second.resize(length, &0);

        let mut borrow = 0;
        let mut new_value: Vec<i32> = first
            .iter()
            .zip(second)
            .map(|(a, b)| {
                let temp = borrow;
                if (*a - temp) < *b {
                    borrow = 1;
                    return 10 + *a - temp - *b;
                }

                borrow = 0;
                return *a - temp - *b;
            })
            .collect();

        let mut is_negative = false;
        if borrow == 1 {
            new_value.pop();
            is_negative = true;
        }

        while new_value.len() > 1 && new_value.last().expect("We have checked for  len") == &0 {
            new_value.pop();
        }
        new_value.reverse();

        BigInt {
            value: new_value,
            is_negative,
            rest: 0,
        }
    }
}

#[cfg(test)]
mod tests_big_int {
    use super::*;
    #[test]
    fn test_case_new() {
        let bi = BigInt::new();
        assert_eq!(bi.value, vec![0]);
    }
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

    // a+b -> a+b
    #[test]
    fn test_case_add() {
        let bi_1 = BigInt::from_str("6786");
        let bi_2 = BigInt::from_str("354");
        let answer = bi_1.add(&bi_2);
        assert_eq!(answer.to_string(), "7140");
    }
    #[test]
    fn test_case_add_2() {
        let bi_1 = BigInt::from_str("354");
        let bi_2 = BigInt::from_str("6786");
        let answer = bi_1.add(&bi_2);
        assert_eq!(answer.to_string(), "7140");
    }

    // -a+b -> b-a
    #[test]
    fn test_case_add_3() {
        let bi_1 = BigInt::from_str("-55");
        let bi_2 = BigInt::from_str("100");
        let answer = bi_1.add(&bi_2);
        assert_eq!(answer.to_string(), "45");
    }

    // a+-b -> a-b
    #[test]
    fn test_case_add_4() {
        let bi_1 = BigInt::from_str("100");
        let bi_2 = BigInt::from_str("-55");
        let answer = bi_1.add(&bi_2);
        assert_eq!(answer.to_string(), "45");
    }

    // -a+-b -> -(a+b)
    #[test]
    fn test_case_add_5() {
        let bi_1 = BigInt::from_str("-30");
        let bi_2 = BigInt::from_str("-20");
        let answer = bi_1.add(&bi_2);
        assert_eq!(answer.to_string(), "-50");
    }

    // a-b  -> a-b
    #[test]
    fn test_case_subtract_1() {
        let bi_1 = BigInt::from_str("100");
        let bi_2 = BigInt::from_str("50");
        let answer = bi_1.subtract(&bi_2);
        assert_eq!(answer.to_string(), "50");
    }

    #[test]
    fn test_case_subtract_2() {
        let bi_1 = BigInt::from_str("50");
        let bi_2 = BigInt::from_str("100");
        let answer = bi_1.subtract(&bi_2);
        assert_eq!(answer.to_string(), "-50");
    }

    // -a--b -> b-a
    #[test]
    fn test_case_subtract_3() {
        let bi_1 = BigInt::from_str("-20");
        let bi_2 = BigInt::from_str("-30");
        let answer = bi_1.subtract(&bi_2);
        assert_eq!(answer.to_string(), "10");
    }

    // a--b -> a+b
    #[test]
    fn test_case_subtract_4() {
        let bi_1 = BigInt::from_str("20");
        let bi_2 = BigInt::from_str("-30");
        let answer = bi_1.subtract(&bi_2);
        assert_eq!(answer.to_string(), "50");
    }

    // -a-b -> -(a+b)
    #[test]
    fn test_case_subtract_5() {
        let bi_1 = BigInt::from_str("-20");
        let bi_2 = BigInt::from_str("30");
        let answer = bi_1.subtract(&bi_2);
        assert_eq!(answer.to_string(), "-50");
    }
}
