use super::BigInt;

impl BigInt {
    fn slow_division(&self, other: &BigInt) -> BigInt {
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
    pub fn division(&self, other: &BigInt) -> BigInt {
        // I need to implement long division
        let mut current = String::new();
        let mut answer = String::new();
        let mut rest = vec![0];
        let positivt_other = BigInt {
            value: other.value.clone(),
            is_negative: false,
            rest: vec![0],
        };
        for x in self.value.iter() {
            current += &x.to_string();
            //self.value.len() <= bi.value.len() && self.to_string() < bi.to_string()
            let mut bi = BigInt::from_str(&current);

            //the rest is worth 10x more on the next round.
            let mut rest2 = rest.clone();
            rest2.push(0);
            let bi_rest = BigInt {
                value: rest2,
                is_negative: false,
                rest: vec![0],
            };
            bi = bi.add(&bi_rest);
            if bi.greater_than(&positivt_other) || bi.to_string() == positivt_other.to_string() {
                let temp = bi.slow_division(other);
                answer += &temp.to_string(); //this should only be one digit
                rest = temp.rest;

                current = String::new();
            } else {
                answer += "0";
            }
        }
        BigInt {
            value: BigInt::from_str(&answer).value,
            is_negative: self.is_negative != other.is_negative,
            rest,
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
        let bi_1 = BigInt::from_str("1015240351");
        let bi_2 = BigInt::from_str("2");
        let answer = bi_1.division(&bi_2);
        assert_eq!(answer.to_string(), "507620175");
        assert_eq!(answer.rest, vec![1]);
    }

    #[test]
    fn test_case_division_5() {
        let bi_1 = BigInt::from_str("101524035174539890485408575671085261788758965189060164484385690801466167356667036677932998889725476582421738788500738738503134356158197247473850273565349249573867251280253564698939768700489401960767007716413932851838937641880157263936985954881657889497583485535527613578457628399173971810541670838543309159139");
        let bi_2 = BigInt::from_str("2");
        let answer = bi_1.division(&bi_2);
        assert_eq!(answer.to_string(), "50762017587269945242704287835542630894379482594530082242192845400733083678333518338966499444862738291210869394250369369251567178079098623736925136782674624786933625640126782349469884350244700980383503858206966425919468820940078631968492977440828944748791742767763806789228814199586985905270835419271654579569");
        assert_eq!(answer.rest, vec![1]);
    }
}
