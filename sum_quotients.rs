use cargo_snippet::snippet;

#[snippet]
/// Σ_{b=1..=n} floor(n / b)
///
/// ### 概要
/// - S(n) = sum_{b=1..=n} floor(n / b)
/// - 計算量: O(√n)
/// - n は最大 10^18 程度まで安全に扱える
/// - 内部計算は u128 を使用しオーバーフロー安全
pub fn sum_quotients(n: usize) -> u128 {
    let nn = n as u128;
    let mut res: u128 = 0;
    let mut b: u128 = 1;

    while b <= nn {
        let v = nn / b; // 商 floor(n/b)
        let rb = nn / v; // 商 v が続く最大 b
        let len = rb - b + 1; // 区間 [b, rb] の長さ

        res += v * len;
        b = rb + 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 愚直計算
    fn naive_sum(n: usize) -> u128 {
        let mut s = 0u128;
        for b in 1..=n {
            s += (n / b) as u128;
        }
        s
    }

    #[test]
    fn test_small_values() {
        for n in 1..=10000 {
            assert_eq!(sum_quotients(n), naive_sum(n));
        }
    }

    #[test]
    fn test_examples() {
        assert_eq!(sum_quotients(1), 1); // floor(1/1)=1
        assert_eq!(sum_quotients(2), 3); // 2/1=2, 2/2=1 →3
        assert_eq!(sum_quotients(10), 27);
        assert_eq!(sum_quotients(100), naive_sum(100));
    }
}
