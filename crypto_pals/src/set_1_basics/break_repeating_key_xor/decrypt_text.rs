use crate::set_1_basics::base64_to_decimal;

pub fn decrypt_text(text: &str, cipher: Vec<char>) -> String {
    let mut answer: String = "".to_string();
    let mut cipher_index = 0;

    let bytes = base64_to_decimal(text);
    for b in bytes {
        let j = cipher[cipher_index];
        let k = (b ^ (j as u8)) as char;
        answer.push(k);
        cipher_index = (cipher_index + 1) % cipher.len();
    }
    answer
}
