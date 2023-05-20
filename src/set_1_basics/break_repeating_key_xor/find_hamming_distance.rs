pub fn find_hamming_distance(a: &[u8], b: &[u8]) -> usize {
    a.iter()
        .zip(b.iter())
        .map(|(a1, b1)| {
            let bin_a1 = format!("{:08b}", a1);
            let bin_b1 = format!("{:08b}", b1);
            bin_a1
                .chars()
                .zip(bin_b1.chars())
                .filter(|(x, y)| *x != *y)
                .count()
        })
        .sum()
}
#[cfg(test)]
mod tests_find_hamming_distance {
    use super::*;

    #[test]
    fn test_case_1() {
        let a: Vec<u8> = vec![
            116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116,
        ];
        let b: Vec<u8> = vec![
            119, 111, 107, 107, 97, 32, 119, 111, 107, 107, 97, 33, 33, 33,
        ];
        let distance = find_hamming_distance(&a, &b);
        assert_eq!(37, distance);
    }
}
