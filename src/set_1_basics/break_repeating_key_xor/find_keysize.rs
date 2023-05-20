use super::{base64_to_decimal, find_hamming_distance};

pub fn find_keysize(first_line: &str) -> usize {
    let mut smallest_distance = usize::MAX;
    let mut best_keysize = 0;

    let first_bytes: Vec<u8> = base64_to_decimal(&first_line);
    for keysize in 1..40 {
        let mut distance = 0;
        let slices = 6;
        for start in 0..slices {
            for end in start + 1..slices + 1 {
                let slice1 = &first_bytes[start * keysize..(start + 1) * keysize];
                let slice2 = &first_bytes[end * keysize..(end + 1) * keysize];
                distance += find_hamming_distance(slice1, slice2) / keysize;
            }
        }

        if smallest_distance > distance {
            smallest_distance = distance;
            best_keysize = keysize;
        }
    }
    best_keysize
}

#[cfg(test)]
mod tests_findkeysize {
    use super::find_keysize;
    use crate::set_1_basics::break_repeating_key_xor::get_lines;

    #[test]
    fn test_case_1() {
        let all_lines = get_lines();
        let keysize = find_keysize(&all_lines);
        assert_eq!(keysize, 29);
    }
}
