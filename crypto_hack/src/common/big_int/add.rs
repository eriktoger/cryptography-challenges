use super::BigInt;

impl BigInt {
    pub fn add(&self, bi: &BigInt) -> BigInt {
        if self.is_negative && bi.is_negative {
            let new_bi = BigInt {
                value: self.value.clone(),
                is_negative: false,
                rest: vec![0],
            };
            return bi.subtract(&new_bi);
        }

        if self.is_negative && !bi.is_negative {
            let new_bi = BigInt {
                value: self.value.clone(),
                is_negative: false,
                rest: vec![0],
            };
            return bi.subtract(&new_bi);
        }

        if !self.is_negative && bi.is_negative {
            let new_bi = BigInt {
                value: bi.value.clone(),
                is_negative: false,
                rest: vec![0],
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
        while new_value.len() > 1 && new_value.last().expect("We have checked for  len") == &0 {
            new_value.pop();
        }

        new_value.reverse();
        return BigInt {
            value: new_value,
            is_negative: false,
            rest: vec![0],
        };
    }
}

#[cfg(test)]
mod tests_big_int_add {
    use super::*;
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
}
