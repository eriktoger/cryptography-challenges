pub fn ascii() -> String {
    let k: [u8; 23] = [
        99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98,
        108, 51, 125,
    ];

    let mut answer = String::new();
    for x in k {
        let c = x as char;
        answer.push(c);
    }
    answer
}

#[cfg(test)]
mod tests_ascii {
    use super::*;
    #[test]
    fn test_case_1() {
        let answer = ascii();
        let password = String::from("crypto{ASCII_pr1nt4bl3}");
        assert_eq!(answer, password);
    }
}
