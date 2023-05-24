const ASCII_ZERO: u8 = 48;

//https://www.wikihow.com/Convert-from-Decimal-to-Binary
fn big_int_to_binary(input: &str) -> String {
    let mut rest = 0;
    let mut current = String::from(input);
    let mut next = String::new();
    let mut answer = String::new();

    while current != "" {
        for c in current.chars() {
            let x = (c as u8) - ASCII_ZERO + rest * 10;
            let a = x / 2;
            rest = x % 2;
            let new_c = (a + ASCII_ZERO) as char;
            let is_leading_zero = next.is_empty() && new_c == '0';
            if !is_leading_zero {
                next.push(new_c);
            }
        }
        answer.push((rest + ASCII_ZERO) as char);
        rest = 0;
        current = next.clone();
        next = String::new();
    }
    answer.chars().rev().collect()
}

fn add_padding(mut binary_string: String) -> String {
    while binary_string.len() % 8 != 0 {
        binary_string = format!("0{binary_string}");
    }
    binary_string
}

fn binary_to_hex_to_char(binary_string: &[char]) -> char {
    let mut x = 0;
    for c in binary_string {
        x = x * 2 + ((*c as u8) - ASCII_ZERO);
    }
    x as char
}

pub fn bytes_and_big_integers() -> String {
    let input = "11515195063862318899931685488813747395775516287289682636499965282714637259206269";
    let binary_string = big_int_to_binary(input);

    let binary_string = add_padding(binary_string);

    binary_string
        .chars()
        .collect::<Vec<_>>()
        .chunks(8)
        .map(binary_to_hex_to_char)
        .collect()
}

#[cfg(test)]
mod tests_bytes_and_big_integers {
    use super::*;
    #[test]
    fn test_case_1() {
        let binary_string = big_int_to_binary("567");
        assert_eq!(binary_string, "1000110111");
    }
    #[test]
    fn test_case_2() {
        let answer = bytes_and_big_integers();
        assert_eq!(answer, "crypto{3nc0d1n6_4ll_7h3_w4y_d0wn}");
    }
}
