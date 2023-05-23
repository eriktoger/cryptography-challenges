pub fn great_snakes() -> String {
    let k: [u8; 21] = [
        81, 64, 75, 66, 70, 93, 73, 72, 1, 92, 109, 2, 84, 109, 66, 75, 70, 90, 2, 92, 79,
    ];

    let mut answer = String::new();
    for x in k {
        let c = (x ^ 0x32) as char;
        answer.push(c);
    }
    answer
}

#[cfg(test)]
mod tests_great_snakes {
    use super::*;
    #[test]
    fn test_case_1() {
        let answer = great_snakes();
        let password = "crypto{z3n_0f_pyth0n}".to_string();

        assert_eq!(answer, password);
    }
}
