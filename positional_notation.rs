use cargo_snippet::snippet;

#[snippet]
pub fn chars_to_decimal(n: Vec<char>, base: usize) -> usize {
    assert!(base >= 2);

    let mut result = 0;
    let mut x = 1;
    for &c in n.iter().rev() {
        result += (c as usize - '0' as usize) * x;
        x *= base;
    }
    result
}

#[snippet]
pub fn decimal_to_chars(mut n: usize, base: usize) -> Vec<char> {
    assert!(base >= 2);

    if n == 0 {
        return vec!['0'];
    }
    let mut result = Vec::new();
    while n > 0 {
        let ch = std::char::from_digit((n % base) as u32, base as u32).unwrap();
        result.push(ch);
        n /= base;
    }
    result.iter().rev().copied().collect()
}

#[snippet]
pub fn num_digits(mut n: usize, base: usize) -> usize {
    assert!(base >= 2);

    if n == 0 {
        return 1;
    }

    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n /= base;
    }
    digits
}

#[snippet]
pub fn pow_base(base: usize, exp: usize) -> usize {
    assert!(base >= 2);

    let mut result = 1usize;
    for _ in 0..exp {
        result *= base;
    }
    result
}

#[snippet(include = "num_digits")]
#[snippet(include = "pow_base")]
pub fn concat_in_base(a: usize, b: usize, base: usize) -> usize {
    a * pow_base(base, num_digits(b, base)) + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chars_to_decimal() {
        assert_eq!(chars_to_decimal(vec!['2', '1'], 8), 17);
        assert_eq!(chars_to_decimal(vec!['1', '8'], 9), 17);
        assert_eq!(chars_to_decimal(vec!['1', '0', '1'], 2), 5);
    }

    #[test]
    fn test_decimal_to_chars() {
        assert_eq!(decimal_to_chars(17, 9), vec!['1', '8']);
        assert_eq!(decimal_to_chars(17, 8), vec!['2', '1']);
        assert_eq!(decimal_to_chars(5, 2), vec!['1', '0', '1']);
        assert_eq!(decimal_to_chars(0, 10), vec!['0']);
    }

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(0, 10), 1);
        assert_eq!(num_digits(9, 10), 1);
        assert_eq!(num_digits(10, 10), 2);
        assert_eq!(num_digits(5, 2), 3);
        assert_eq!(num_digits(8, 2), 4);
    }

    #[test]
    fn test_pow_base() {
        assert_eq!(pow_base(10, 0), 1);
        assert_eq!(pow_base(10, 3), 1000);
        assert_eq!(pow_base(2, 5), 32);
    }

    #[test]
    fn test_concat_in_base() {
        assert_eq!(concat_in_base(12, 34, 10), 1234);
        assert_eq!(concat_in_base(999, 0, 10), 9990);
        assert_eq!(concat_in_base(0b101, 0b11, 2), 0b10111);
        assert_eq!(concat_in_base(0b1, 0b0, 2), 0b10);
    }
}
