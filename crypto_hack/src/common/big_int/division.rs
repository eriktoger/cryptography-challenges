use super::BigInt;

impl BigInt {
    pub fn division(&self, other: &BigInt) -> BigInt {
        let mut self_copy = BigInt {
            value: self.value.clone(),
            is_negative: false,
            rest: vec![0],
        };

        let mut answer = BigInt::new();
        let one = BigInt::from_int(1).expect("");
        while !self_copy.is_negative {
            self_copy = self_copy.subtract(other);
            answer = answer.add(&one);
        }

        //we went one step to far
        self_copy = self_copy.add(other);
        answer = answer.subtract(&one);

        BigInt {
            value: answer.value,
            is_negative: self.is_negative != other.is_negative,
            rest: self_copy.value,
        }
    }
}

#[cfg(test)]
mod tests_big_int_division {
    use super::*;
    #[test]
    fn test_case_division_1() {
        let bi_1 = BigInt::from_int(25).expect("");
        let bi_2 = BigInt::from_int(5).expect("");
        let answer = bi_1.division(&bi_2);
        assert_eq!(answer.to_string(), "5");
    }

    #[test]
    fn test_case_division_2() {
        let bi_1 = BigInt::from_int(25).expect("");
        let bi_2 = BigInt::from_int(4).expect("");
        let answer = bi_1.division(&bi_2);
        assert_eq!(answer.to_string(), "6");
        assert_eq!(answer.rest, vec![1]);
    }
    #[test]
    fn test_case_division_3() {
        let bi_1 = BigInt::from_str("5454545453");
        let bi_2 = BigInt::from_str("3944");
        let answer = bi_1.division(&bi_2);
        assert_eq!(answer.to_string(), "1382998");
        assert_eq!(answer.rest, vec![1, 3, 4, 1]);
    }

    #[test]
    fn test_case_division_4() {
        let bi_1 = BigInt::from_str("101524035174539890485408575671085261788758965189060164484385690801466167356667036677932998889725476582421738788500738738503134356158197247473850273565349249573867251280253564698939768700489401960767007716413932851838937641880157263936985954881657889497583485535527613578457628399173971810541670838543309159139");
        let bi_2 = BigInt::from_str("2");
        let answer = bi_1.division(&bi_2);
        assert_eq!(answer.to_string(), "1382998");
        assert_eq!(answer.rest, vec![1, 3, 4, 1]);
    }
}
