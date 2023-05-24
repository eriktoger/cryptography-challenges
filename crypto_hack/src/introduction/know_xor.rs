use crate::common::hex_to_decimals;

pub fn know_xor() -> String {
    let decimals = hex_to_decimals(
        "0e0b213f26041e480b26217f27342e175d0e070a3c5b103e2526217f27342e175d0e077e263451150104",
    );

    let key: Vec<u8> = vec![
        99 ^ 14,
        114 ^ 11,
        121 ^ 33,
        112 ^ 63,
        116 ^ 38,
        111 ^ 4,
        123 ^ 30,
        121,
    ];
    // myXORkey

    let mut key_index: usize = 0;
    let mut answer = String::new();
    for x in decimals {
        let c = (x ^ key[key_index]) as char;
        answer.push(c);
        key_index = (key_index + 1) % key.len();
    }
    answer
}

#[cfg(test)]
mod tests_know_xor {
    use super::*;
    #[test]
    fn test_case_1() {
        let answer = know_xor();
        let password = String::from("crypto{1f_y0u_Kn0w_En0uGH_y0u_Kn0w_1t_4ll}");
        assert_eq!(answer, password);
    }
}
