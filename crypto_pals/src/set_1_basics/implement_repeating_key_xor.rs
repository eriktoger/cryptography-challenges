pub fn implement_repeating_key_xor(s: &str, key: &str) -> String {
    let mut key_index: usize = 0;
    let key_len = key.len();

    let mut answer = vec![];

    for c in s.chars() {
        let current_key = key.chars().nth(key_index).unwrap();
        answer.push((c as u32) ^ (current_key as u32));
        key_index = (key_index + 1) % key_len;
    }
    answer
        .iter()
        .map(|c| format!("{:x}", c))
        .map(|s| if s.len() == 1 { format!("0{s}") } else { s })
        .collect()
}

#[cfg(test)]
mod tests_implement_repeating_key_xor {
    use super::*;
    #[test]
    fn test_case_1() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let output = implement_repeating_key_xor(input, key);
        let answer = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
            .to_string();
        assert_eq!(answer, output);
    }
}
