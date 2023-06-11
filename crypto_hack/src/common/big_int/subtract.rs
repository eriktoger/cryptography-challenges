use super::BigInt;

impl BigInt {
    pub fn subtract(&self, bi: &BigInt) -> BigInt {
        if bi.is_negative {
            let new_bi = BigInt {
                value: bi.value.clone(),
                is_negative: false,
                rest: vec![0],
            };
            return self.add(&new_bi);
        }

        if self.is_negative {
            let new_bi = BigInt {
                value: self.value.clone(),
                is_negative: false,
                rest: vec![0],
            };

            return BigInt {
                value: new_bi.add(&bi).value,
                is_negative: true,
                rest: vec![0],
            };
        }

        //le, gt, eq should be functions in a file called compares.rs
        if bi.greater_than(self) {
            return BigInt {
                value: bi.subtract(&self).value,
                is_negative: true,
                rest: vec![0],
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
            rest: vec![0],
        }
    }
}

#[cfg(test)]
mod tests_big_int_multiply {
    use super::*;

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

    #[test]
    fn test_case_subtract_2b() {
        let bi_1 = BigInt::from_str("11");
        let bi_2 = BigInt::from_str("44");
        let answer = bi_1.subtract(&bi_2);
        assert_eq!(answer.to_string(), "-33");
    }
    #[test]
    fn test_case_subtract_2c() {
        let bi_1 = BigInt::from_str("6");
        let bi_2 = BigInt::from_str("13");
        let answer = bi_1.subtract(&bi_2);
        assert_eq!(answer.to_string(), "-7");
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
    // -a-b -> -(a+b)
}
