use cargo_snippet::snippet;

/// mod 付きの通常の行列積:
/// C = A * B
/// C[i][j] = sum_t A[i][t] * B[t][j] (mod modulo)
///
/// 使う場面:
/// - 通り数 DP
/// - 総和 DP
/// - `dp_next[j] = sum_i dp[i] * trans[i][j]` のような線形遷移
///
/// 注意:
/// - 最短距離や最小コストの `min(a+b)` ではない。
///   その場合は `tropical_matrix` の min-plus 行列を使う。
/// - A は n x m、B は m x p。返り値は n x p。
/// - 乗算は `u128` に逃がしてから `modulo` を取るので、`usize` の積 overflow を避ける。
#[snippet]
pub fn mod_mat_mul(a: &[Vec<usize>], b: &[Vec<usize>], modulo: usize) -> Vec<Vec<usize>> {
    debug_assert!(modulo > 0);

    let n = a.len();
    debug_assert!(n > 0);
    let m = a[0].len();
    debug_assert!(a.iter().all(|row| row.len() == m));

    debug_assert!(b.len() == m);
    let p = b[0].len();
    debug_assert!(b.iter().all(|row| row.len() == p));

    let modulo_u128 = modulo as u128;
    let mut ret = vec![vec![0; p]; n];

    for i in 0..n {
        for t in 0..m {
            let x = a[i][t] % modulo;
            if x == 0 {
                continue;
            }
            for j in 0..p {
                let y = b[t][j] % modulo;
                if y == 0 {
                    continue;
                }
                ret[i][j] = ((ret[i][j] as u128 + x as u128 * y as u128) % modulo_u128) as usize;
            }
        }
    }

    ret
}

/// mod 付き通常行列積の単位行列。
///
/// `mod_mat_pow` の初期値として使う。
/// I[i][i] = 1, I[i][j] = 0 (i != j)
#[snippet]
pub fn mod_mat_identity(n: usize, modulo: usize) -> Vec<Vec<usize>> {
    debug_assert!(modulo > 0);

    let mut id = vec![vec![0; n]; n];
    for i in 0..n {
        id[i][i] = 1 % modulo;
    }
    id
}

/// mod 付きの通常の行列累乗。
///
/// `A^exp` を返す。積は `mod_mat_mul` と同じ通常積:
/// C[i][j] = sum_t A[i][t] * B[t][j] (mod modulo)
///
/// 使う場面:
/// - 同じ線形遷移を `exp` 回繰り返す DP
/// - `dp[t+1] = dp[t] * A` なら、`dp[N] = dp[0] * A^N`
///
/// 前提:
/// - A は正方行列。
/// - exp = 0 のときは単位行列を返す。
#[snippet(include = "mod_mat_mul,mod_mat_identity")]
pub fn mod_mat_pow(mut a: Vec<Vec<usize>>, mut exp: usize, modulo: usize) -> Vec<Vec<usize>> {
    let n = a.len();
    debug_assert!(n > 0);
    debug_assert!(a.iter().all(|row| row.len() == n));

    let mut res = mod_mat_identity(n, modulo);
    while exp > 0 {
        if (exp & 1) == 1 {
            res = mod_mat_mul(&res, &a, modulo);
        }
        a = mod_mat_mul(&a, &a, modulo);
        exp >>= 1;
    }

    res
}

/// 行ベクトルと行列の mod 付き積:
/// ret[j] = sum_i v[i] * A[i][j] (mod modulo)
///
/// DP を行ベクトルで持つときに使う。
///
/// 例:
/// - `dp[state] = 現在その状態にいる通り数`
/// - `trans[i][j] = 状態 i から状態 j へ行く遷移数`
/// - `next = mod_vec_mat_mul(&dp, &trans, MOD)`
#[snippet]
pub fn mod_vec_mat_mul(v: &[usize], a: &[Vec<usize>], modulo: usize) -> Vec<usize> {
    debug_assert!(modulo > 0);

    let n = v.len();
    debug_assert!(a.len() == n);
    let m = a[0].len();
    debug_assert!(a.iter().all(|row| row.len() == m));

    let modulo_u128 = modulo as u128;
    let mut ret = vec![0; m];
    for i in 0..n {
        let x = v[i] % modulo;
        if x == 0 {
            continue;
        }
        for j in 0..m {
            let y = a[i][j] % modulo;
            if y == 0 {
                continue;
            }
            ret[j] = ((ret[j] as u128 + x as u128 * y as u128) % modulo_u128) as usize;
        }
    }

    ret
}

/// 行ベクトル `v` に行列 `A^exp` を掛ける。
///
/// `A^exp` を明示的に作らず、二分累乗の途中で bit が立ったときだけ
/// `v = v * A` を行う。
///
/// 使う場面:
/// - 初期 DP が 1 本のベクトルで、最終ベクトルだけ欲しいとき。
/// - `dp[N] = dp[0] * trans^N` をそのまま計算したいとき。
///
/// 前提:
/// - A は正方行列。
/// - v.len() == A.len()
#[snippet(include = "mod_mat_mul,mod_vec_mat_mul")]
pub fn mod_vec_mat_pow_mul(
    mut v: Vec<usize>,
    mut a: Vec<Vec<usize>>,
    mut exp: usize,
    modulo: usize,
) -> Vec<usize> {
    let n = a.len();
    debug_assert!(n > 0);
    debug_assert!(a.iter().all(|row| row.len() == n));
    debug_assert!(v.len() == n);

    while exp > 0 {
        if (exp & 1) == 1 {
            v = mod_vec_mat_mul(&v, &a, modulo);
        }
        a = mod_mat_mul(&a, &a, modulo);
        exp >>= 1;
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOD: usize = 998244353;

    #[test]
    fn test_rectangular_mul_shape_and_values() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let b = vec![vec![7, 8], vec![9, 10], vec![11, 12]];

        let c = mod_mat_mul(&a, &b, MOD);
        assert_eq!(c, vec![vec![58, 64], vec![139, 154]]);
    }

    #[test]
    fn test_identity_left_right_square() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let id = mod_mat_identity(3, MOD);

        assert_eq!(mod_mat_mul(&id, &a, MOD), a);
        assert_eq!(mod_mat_mul(&a, &id, MOD), a);
    }

    #[test]
    fn test_pow_0_is_identity() {
        let a = vec![vec![1, 1], vec![1, 0]];
        assert_eq!(mod_mat_pow(a, 0, MOD), mod_mat_identity(2, MOD));
    }

    #[test]
    fn test_fibonacci_matrix_pow() {
        // [[1, 1], [1, 0]]^5 = [[F6, F5], [F5, F4]]
        let a = vec![vec![1, 1], vec![1, 0]];
        assert_eq!(mod_mat_pow(a, 5, MOD), vec![vec![8, 5], vec![5, 3]]);
    }

    #[test]
    fn test_vec_mat_mul() {
        let v = vec![2, 3];
        let a = vec![vec![5, 7], vec![11, 13]];

        assert_eq!(mod_vec_mat_mul(&v, &a, MOD), vec![43, 53]);
    }

    #[test]
    fn test_vec_mat_pow_mul_matches_repeated_application() {
        let trans = vec![vec![1, 1], vec![1, 0]];
        let mut repeated = vec![1, 0];
        for _ in 0..5 {
            repeated = mod_vec_mat_mul(&repeated, &trans, MOD);
        }

        let by_pow = mod_vec_mat_pow_mul(vec![1, 0], trans, 5, MOD);
        assert_eq!(by_pow, repeated);
        assert_eq!(by_pow, vec![8, 5]);
    }
}
