pub fn entry_cipher() -> Vec<String> {
    let s = "FWBU DFCPZSA GSZZ WZZ";
    let mut answers = vec![];
    for shift in 1..27 {
        let mut answer = String::new();
        for c in s.chars() {
            if c == ' ' {
                answer.push(c);
                continue;
            }
            let mut ch = (c as u8) + shift;
            if ch > 90 {
                ch -= 26
            }
            answer.push(ch as char);
        }
        answers.push(answer);
    }
    println!("{answers:?}");
    answers
}

#[cfg(test)]
mod tests_entry_cipher {
    use super::*;
    #[test]
    fn test_case_1() {
        let answers = entry_cipher();
        let password = "RING PROBLEM SELL ILL".to_string();
        let found_it = answers.iter().find(|s| **s == password).is_some();
        assert_eq!(found_it, true);
    }
}
