use cargo_snippet::snippet;

#[snippet]
/// f64の仮数部は53bitなので、nが2^53を超えると誤差が生まれる
/// そのため、u64の平方根を求める関数を自前で実装する必要がある
/// https://rsk0315.hatenablog.com/entry/2023/11/07/221428
pub fn u64_floor_sqrt(n: u64) -> u64 {
    let tmp = (n as f64).sqrt() as u64;
    let tmp_m1 = tmp.saturating_sub(1);
    if tmp_m1 * (tmp_m1 + 2) < n {
        tmp
    } else {
        tmp_m1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u64_floor_sqrt() {
        let test_cases = vec![
            (0, 0),
            (1, 1),
            ((1 << 26) * (1 << 26) - 1, (1 << 26) - 1),
            ((1 << 26) * (1 << 26), 1 << 26),
            (((1 << 26) + 1) * ((1 << 26) + 1) - 1, (1 << 26)),
            (((1 << 26) + 1) * ((1 << 26) + 1), (1 << 26) + 1),
            (u64::MAX, (1u64 << 32) - 1), // 2^64 - 1 の平方根は 2^32 - 1
        ];

        for (input, expected) in test_cases {
            assert_eq!(u64_floor_sqrt(input), expected, "Failed on input {}", input);
        }
    }
}
