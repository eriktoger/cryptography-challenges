use super::BigInt;

impl BigInt {
    fn base_case_multiplication(b1: &BigInt, b2: &BigInt, is_negative: bool) -> BigInt {
        let (single_digit, multiple_digits) = if b1.value.len() == 1 {
            (b1, b2)
        } else {
            (b2, b1)
        };

        let mut rest = 0;
        let mut result = vec![];
        for x in multiple_digits.value.iter().rev() {
            let r = x * single_digit.value[0] + rest;
            rest = r / 10;
            result.push(r % 10);
        }
        if rest != 0 {
            result.push(rest)
        }
        result.reverse();
        return BigInt {
            value: result,
            is_negative,
            rest: vec![0],
        };
    }

    pub fn multiply(&self, other: &BigInt) -> BigInt {
        let len_1 = self.value.len();
        let len_2 = other.value.len();
        let is_negative = self.is_negative != other.is_negative;

        if len_1 == 1 || len_2 == 1 {
            return BigInt::base_case_multiplication(self, other, is_negative);
        }

        let mut s_value = self.value.clone();
        let mut bi_value = other.value.clone();

        let size = usize::max(len_1, len_2);

        while s_value.len() < size {
            let mut v = vec![0];
            v.extend(s_value.iter());
            s_value = v;
        }

        while bi_value.len() < size {
            let mut v = vec![0];
            v.extend(s_value.iter());
            bi_value = v;
        }

        let mid = size / 2;
        let short = size - mid;

        //maybe create a helper that returns high_self as big_int?
        //that function can also pad?
        let (high_self, low_self) = s_value.split_at(mid);
        let (high_other, low_other) = bi_value.split_at(mid);

        let b1 = BigInt {
            value: low_self.to_vec(),
            is_negative: false,
            rest: vec![0],
        };

        let b2 = BigInt {
            value: low_other.to_vec(),
            is_negative: false,
            rest: vec![0],
        };
        let z0 = b1.multiply(&b2);

        let b5 = BigInt {
            value: high_self.to_vec(),
            is_negative: false,
            rest: vec![0],
        };
        let b6 = BigInt {
            value: high_other.to_vec(),
            is_negative: false,
            rest: vec![0],
        };

        let z2 = b5.multiply(&b6);

        let b3 = b1.add(&b5);
        let b4 = b2.add(&b6);

        let z1 = b3.multiply(&b4);

        let mut appended_z2 = z2.value.clone();
        for _ in 0..(short * 2) {
            appended_z2.push(0);
        }

        let mut diff = z1.subtract(&z2).subtract(&z0).value;

        for _ in 0..(short) {
            diff.push(0);
        }

        let r1 = BigInt {
            value: appended_z2,
            is_negative: false,
            rest: vec![0],
        };
        let r2 = BigInt {
            value: diff,
            is_negative: false,
            rest: vec![0],
        };

        let r3 = z0;

        let result = r1.add(&r2).add(&r3);

        BigInt {
            value: result.value,
            is_negative,
            rest: vec![0],
        }
    }
}

#[cfg(test)]
mod tests_big_int_multiply {
    use super::*;

    #[test]
    fn test_case_mulitply_1() {
        let bi_1 = BigInt::from_int(5).expect("");
        let bi_2 = BigInt::from_int(5).expect("");
        let answer = bi_1.multiply(&bi_2);
        assert_eq!(answer.to_string(), "25");
    }

    #[test]
    fn test_case_mulitply_2() {
        let bi_1 = BigInt::from_int(-5).expect("");
        let bi_2 = BigInt::from_int(5).expect("");
        let answer = bi_1.multiply(&bi_2);
        assert_eq!(answer.to_string(), "-25");
    }
    #[test]
    fn test_case_mulitply_3() {
        let bi_1 = BigInt::from_int(-5).expect("");
        let bi_2 = BigInt::from_int(-5).expect("");
        let answer = bi_1.multiply(&bi_2);
        assert_eq!(answer.to_string(), "25");
    }

    #[test]
    fn test_case_mulitply_4() {
        let bi_1 = BigInt::from_int(15).expect("");
        let bi_2 = BigInt::from_int(12).expect("");
        let answer = bi_1.multiply(&bi_2);
        assert_eq!(answer.to_string(), "180");
    }
    #[test]
    fn test_case_mulitply_5() {
        let bi_1 = BigInt::from_int(-15).expect("");
        let bi_2 = BigInt::from_int(12).expect("");
        let answer = bi_1.multiply(&bi_2);
        assert_eq!(answer.to_string(), "-180");
    }

    #[test]
    fn test_case_mulitply_6() {
        let bi_1 = BigInt::from_int(-15).expect("");
        let bi_2 = BigInt::from_int(-12).expect("");
        let answer = bi_1.multiply(&bi_2);
        assert_eq!(answer.to_string(), "180");
    }

    #[test]
    fn test_case_mulitply_7() {
        let bi_1 = BigInt::from_str("123");
        let bi_2 = BigInt::from_str("987");
        let answer = bi_1.multiply(&bi_2);
        assert_eq!(answer.to_string(), "121401");
    }

    #[test]
    fn test_case_mulitply_7b() {
        let bi_1 = BigInt::from_str("15");
        let bi_2 = BigInt::from_str("15");
        let answer = bi_1.multiply(&bi_2);
        assert_eq!(answer.to_string(), "225");
    }
    #[test]
    fn test_case_mulitply_7a() {
        let bi_1 = BigInt::from_str("15");
        let bi_2 = BigInt::from_str("105");
        let answer = bi_1.multiply(&bi_2);
        assert_eq!(answer.to_string(), "1575");
    }
    #[test]
    fn test_case_mulitply_8() {
        let bi_1 = BigInt::from_str("5");
        let bi_2 = BigInt::from_str("987");
        let answer = bi_1.multiply(&bi_2);
        assert_eq!(answer.to_string(), "4935");
    }

    #[test]
    fn test_case_mulitply_9() {
        let bi_1 = BigInt::from_str("12345678901234567890");
        let bi_2 = BigInt::from_str("98765432109876543210");
        let answer = bi_1.multiply(&bi_2);
        assert_eq!(
            answer.to_string(),
            "1219326311370217952237463801111263526900"
        );
    }

    #[test]
    fn test_case_mulitply_10() {
        let bi_1 = BigInt::from_str("50762017587269945242704287835542630894379482594530082242192845400733083678333518338966499444862738291210869394250369369251567178079098623736925136782674624786933625640126782349469884350244700980383503858206966425919468820940078631968492977440828944748791742767763806789228814199586985905270835419271654579569");
        let bi_2 = BigInt::from_str("2");
        let answer = bi_1.multiply(&bi_2);
        assert_eq!(answer.to_string(), "101524035174539890485408575671085261788758965189060164484385690801466167356667036677932998889725476582421738788500738738503134356158197247473850273565349249573867251280253564698939768700489401960767007716413932851838937641880157263936985954881657889497583485535527613578457628399173971810541670838543309159138");
    }
}
