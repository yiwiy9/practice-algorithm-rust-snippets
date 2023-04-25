use cargo_snippet::snippet;

#[snippet]
pub fn chars_to_decimal(n: Vec<char>, base: usize) -> usize {
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
    if n == 0 {
        return vec!['0'];
    }
    let mut result = Vec::new();
    while n > 0 {
        let char = std::char::from_digit((n % base) as u32, base as u32).unwrap();
        result.push(char);
        n /= base;
    }
    result.iter().rev().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chars_to_decimal() {
        assert_eq!(chars_to_decimal(vec!['2', '1'], 8), 17)
    }

    #[test]
    fn test_decimal_to_chars() {
        assert_eq!(decimal_to_chars(17, 9), vec!['1', '8'])
    }
}
