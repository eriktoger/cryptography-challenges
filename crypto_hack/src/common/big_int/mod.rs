mod add;
mod multiply;
mod subtract;
mod transform;

pub struct BigInt {
    value: Vec<i32>,
    is_negative: bool,
    rest: i32,
}

impl BigInt {
    pub fn new() -> BigInt {
        BigInt {
            value: vec![0],
            is_negative: false,
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
}