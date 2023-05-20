use hex::{decode, encode};
pub fn xor(a: &str, b: &str) -> String {
    let a_bytes = decode(a).unwrap();
    let b_bytes = decode(b).unwrap();

    let xor_bytes: Vec<u8> = a_bytes
        .iter()
        .zip(b_bytes.iter())
        .map(|(&a, &b)| a ^ b)
        .collect();
    encode(xor_bytes)
}

#[cfg(test)]
mod tests_xor {
    use super::*;
    #[test]
    fn test_case_1() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        assert_eq!(xor(a, b), "746865206b696420646f6e277420706c6179");
    }
}
