pub fn xor_starter() -> String {
    let input = "label";

    let mut answer = String::new();
    for c in input.chars() {
        let x = c as u8;
        let ch = (x ^ 13) as char;
        answer.push(ch)
    }

    answer
}

#[cfg(test)]
mod tests_xor_starter {
    use super::*;
    #[test]
    fn test_case_1() {
        let answer = xor_starter();
        let password = String::from("aloha");
        assert_eq!(answer, password);
    }
}
