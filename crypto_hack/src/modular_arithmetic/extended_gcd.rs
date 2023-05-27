pub fn extended_gcd(mut a: i32, mut b: i32) -> (i32, i32) {
    let mut numbers_a = vec![];
    let mut numbers_b = vec![];
    while b != 0 {
        numbers_a.push(a);
        numbers_b.push(b);

        let temp = b;
        b = a % b;
        a = temp;
    }
    numbers_a.pop();
    numbers_b.pop();

    let combined: Vec<(&i32, i32)> = numbers_a.iter().zip(numbers_b).collect();

    let mut x = 1;
    let mut y = 1;
    for (a, b) in combined.iter().rev() {
        y = x;
        x = (1 - y * (*a)) / b;
    }

    (x, y)
}

#[cfg(test)]
mod tests_extended_gcd {
    use super::*;
    #[test]
    fn test_case_1() {
        let (x, y) = extended_gcd(32321, 26513);
        assert_eq!(x, 10245);
        assert_eq!(y, -8404);
    }
}
