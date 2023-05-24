use crate::common::hex_to_decimals;

pub fn xor_properties() -> String {
    let key_1 = hex_to_decimals("a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313");
    let k2_xor_k3 = hex_to_decimals("c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1");
    let flag_xor_k1_xor_k2_xor_k3 =
        hex_to_decimals("04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf");

    // f ^k1 ^k2 ^ k3 = 04
    // f = 04 ^(k1) ^ (k2^k3)
    key_1
        .iter()
        .zip(k2_xor_k3)
        .zip(flag_xor_k1_xor_k2_xor_k3)
        .map(|((a, b), c)| (a ^ b ^ c) as char)
        .collect()
}

#[cfg(test)]
mod tests_xor_properties {
    use super::*;
    #[test]
    fn test_case_1() {
        let answer = xor_properties();
        let password = String::from("crypto{x0r_i5_ass0c1at1v3}");
        assert_eq!(answer, password);
    }
}
