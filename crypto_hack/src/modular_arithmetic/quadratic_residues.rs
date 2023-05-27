pub fn quadratic_residues() -> i32 {
    let p = 29;
    let ints = [14, 6, 11];
    let mut smallest_qr = i32::MAX;
    for int in ints {
        for i in 1..p {
            if i * i % p == int && i < smallest_qr {
                smallest_qr = i;
            }
        }
    }
    smallest_qr
}

#[cfg(test)]
mod tests_quadratic_residues {
    use super::*;
    #[test]
    fn test_case_1() {
        let answer = quadratic_residues();
        assert_eq!(answer, 8);
    }
}
