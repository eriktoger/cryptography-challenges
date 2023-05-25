pub fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

#[cfg(test)]
mod tests_gcd {
    use super::*;
    #[test]
    fn test_case_1() {
        let answer = gcd(66528, 52920);
        assert_eq!(answer, 1512);
    }
}
