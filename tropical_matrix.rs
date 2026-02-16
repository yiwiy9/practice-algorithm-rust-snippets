use cargo_snippet::snippet;

/// min-plus（tropical）用の「無限大」
///
/// 衝突しにくいように prefix を付ける。
/// (1<<60) を使い、saturating_add + min(INF) で overflow を避ける。
#[snippet]
const TROPICAL_MINPLUS_INF_USIZE: usize = 1usize << 60;

/// min-plus（tropical）積（一般形）:
/// C = A ⊗ B
/// C[i][j] = min_t (A[i][t] + B[t][j])
///
/// 次元:
/// A: n×m, B: m×p, C: n×p
#[snippet(include = "TROPICAL_MINPLUS_INF_USIZE")]
pub fn tropical_min_plus_mul(a: &[Vec<usize>], b: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = a.len();
    debug_assert!(n > 0);
    let m = a[0].len();
    debug_assert!(a.iter().all(|row| row.len() == m));

    debug_assert!(b.len() == m);
    let p = b[0].len();
    debug_assert!(b.iter().all(|row| row.len() == p));

    let mut ret = vec![vec![TROPICAL_MINPLUS_INF_USIZE; p]; n];

    for i in 0..n {
        for t in 0..m {
            let x = a[i][t];
            if x >= TROPICAL_MINPLUS_INF_USIZE {
                continue;
            }
            for j in 0..p {
                let y = b[t][j];
                if y >= TROPICAL_MINPLUS_INF_USIZE {
                    continue;
                }
                let v = x.saturating_add(y).min(TROPICAL_MINPLUS_INF_USIZE);
                if v < ret[i][j] {
                    ret[i][j] = v;
                }
            }
        }
    }

    ret
}

/// min-plus（tropical）の単位行列（N×N）:
/// I[i][i] = 0, I[i][j] = INF (i != j)
///
/// ※ 累乗で必要になるのはこの「正方」単位行列。
#[snippet(include = "TROPICAL_MINPLUS_INF_USIZE")]
fn tropical_min_plus_identity(n: usize) -> Vec<Vec<usize>> {
    let mut id = vec![vec![TROPICAL_MINPLUS_INF_USIZE; n]; n];
    for i in 0..n {
        id[i][i] = 0;
    }
    id
}

/// min-plus（tropical）行列累乗（繰り返し二乗法）:
/// A^(exp)（⊗ による累乗）を返す。
///
/// 前提: A は N×N（正方行列）
/// exp=0 のときは単位行列を返す。
#[snippet(include = "TROPICAL_MINPLUS_INF_USIZE,tropical_min_plus_mul,tropical_min_plus_identity")]
pub fn tropical_min_plus_pow(mut a: Vec<Vec<usize>>, mut exp: usize) -> Vec<Vec<usize>> {
    let n = a.len();
    debug_assert!(a.iter().all(|row| row.len() == n)); // 正方チェック

    let mut res = tropical_min_plus_identity(n);

    while exp > 0 {
        if (exp & 1) == 1 {
            res = tropical_min_plus_mul(&res, &a);
        }
        a = tropical_min_plus_mul(&a, &a);
        exp >>= 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangular_mul_shape_and_values() {
        // A: 2x3, B: 3x2 => C: 2x2
        // INF も混ぜて「不可能」が素直に落ちることを確認
        let inf = TROPICAL_MINPLUS_INF_USIZE;
        let a = vec![vec![0, 5, inf], vec![2, inf, 1]];
        let b = vec![vec![7, 1], vec![inf, 3], vec![4, 0]];

        let c = tropical_min_plus_mul(&a, &b);
        assert_eq!(c.len(), 2);
        assert_eq!(c[0].len(), 2);

        // c[0][0] = min_t(a[0][t]+b[t][0]) = min(0+7, 5+inf, inf+4) = 7
        assert_eq!(c[0][0], 7);
        // c[1][1] = min(2+1, inf+3, 1+0) = 1
        assert_eq!(c[1][1], 1);
    }

    #[test]
    fn test_identity_left_right_square() {
        let a = vec![vec![0, 5, 2], vec![7, 0, 1], vec![3, 9, 0]];
        let id = tropical_min_plus_identity(3);

        assert_eq!(tropical_min_plus_mul(&id, &a), a);
        assert_eq!(tropical_min_plus_mul(&a, &id), a);
    }

    #[test]
    fn test_pow_0_is_identity() {
        let a = vec![vec![0, 1, 2], vec![3, 0, 4], vec![5, 6, 0]];
        let got = tropical_min_plus_pow(a, 0);
        let want = tropical_min_plus_identity(3);
        assert_eq!(got, want);
    }

    #[test]
    fn test_pow_2_matches_mul() {
        let a = vec![vec![0, 2, 10], vec![1, 0, 3], vec![4, 7, 0]];
        let a2_by_mul = tropical_min_plus_mul(&a, &a);
        let a2_by_pow = tropical_min_plus_pow(a, 2);
        assert_eq!(a2_by_pow, a2_by_mul);
    }
}
