use super::BigInt;

impl BigInt {
    pub fn pow_mod(&self, y: &BigInt, p: &BigInt) -> BigInt {
        let mut answer = BigInt::from_str("1");
        let mut y_copy = BigInt {
            value: y.value.clone(),
            is_negative: false,
            rest: vec![0],
        };
        let mut self_copy = BigInt {
            value: self.value.clone(),
            is_negative: false,
            rest: vec![0],
        };

        let two = BigInt::from_str("2");
        while y_copy.to_string() != "0" {
            if y_copy.is_odd() {
                answer = answer.multiply(&self_copy);

                let rest = answer.division(p).rest;

                answer = BigInt {
                    value: rest,
                    is_negative: false,
                    rest: vec![0],
                };
            }
            y_copy = y_copy.division(&two);
            self_copy = self_copy.multiply(&self_copy);
            let rest = self_copy.division(&p).rest;

            self_copy = BigInt {
                value: rest,
                is_negative: false,
                rest: vec![0],
            };
        }
        // % should have its own file

        //answer
        BigInt {
            value: answer.division(p).rest,
            is_negative: false,
            rest: vec![0],
        }
    }
}

#[cfg(test)]
mod tests_big_int_pow_mod {
    use super::*;
    #[test]
    fn test_case_pow_mod_1() {
        let x = BigInt::from_int(2).expect("");
        let y = BigInt::from_int(5).expect("");
        let p = BigInt::from_int(13).expect("");
        let answer = x.pow_mod(&y, &p);
        assert_eq!(answer.to_string(), "6");
    }

    #[test]
    fn test_case_pow_mod_2() {
        let x = BigInt::from_int(26).expect("");
        let y = BigInt::from_int(23).expect("");
        let p = BigInt::from_int(47).expect("");
        let answer = x.pow_mod(&y, &p);
        assert_eq!(answer.to_string(), "46");
    }
    #[test]
    fn test_case_pow_mod_2b() {
        let x = BigInt::from_int(98).expect("");
        let y = BigInt::from_int(165).expect("");
        let p = BigInt::from_int(331).expect("");
        let answer = x.pow_mod(&y, &p);
        assert_eq!(answer.to_string(), "330");
    }
    //SLOW test! Used to solve: https://cryptohack.org/courses/modular/root1/
    // const HACK_P:&str = "101524035174539890485408575671085261788758965189060164484385690801466167356667036677932998889725476582421738788500738738503134356158197247473850273565349249573867251280253564698939768700489401960767007716413932851838937641880157263936985954881657889497583485535527613578457628399173971810541670838543309159139";
    // #[test]
    // fn test_case_pow_mod_3() {
    //     let x = BigInt::from_str("85256449776780591202928235662805033201684571648990042997557084658000067050672130152734911919581661523957075992761662315262685030115255938352540032297113615687815976039390537716707854569980516690246592112936796917504034711418465442893323439490171095447109457355598873230115172636184525449905022174536414781771");
    //     let p = BigInt::from_str(HACK_P);
    //     let one = BigInt::from_str("1");
    //     let two = BigInt::from_str("2");
    //     let p_half: BigInt = p.subtract(&one).division(&two);

    //     let answer = x.pow_mod(&p_half, &p);
    //     assert_eq!(answer.to_string(), "1");
    // }
}
