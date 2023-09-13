use cargo_snippet::snippet;

#[snippet("ModComb")]
pub struct ModComb {
    modulo: usize,
    fac: Vec<usize>,
    finv: Vec<usize>,
}

#[snippet("ModComb")]
impl ModComb {
    // テーブルを作る前処理
    // a^(-1) ≡ -(p%a)^(-1) * (p/a)   (mod.p)
    pub fn new(cap: usize, modulo: usize) -> Self {
        let mut fac = vec![0; cap];
        let mut finv = vec![0; cap];
        let mut inv = vec![0; cap];
        fac[0] = 1;
        fac[1] = 1;
        finv[0] = 1;
        finv[1] = 1;
        inv[1] = 1;
        for i in 2..cap {
            fac[i] = fac[i - 1] * i % modulo;
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo; // 負の数の余りが負になるので、割る数を足して正にする
            finv[i] = finv[i - 1] * inv[i] % modulo;
        }

        Self { modulo, fac, finv }
    }

    // 二項係数計算
    // nCk = n!/(k!(n-k)!) = (n!) * (k!)^(-1) * ((n-k))!)^(-1)
    //
    // 参考：https://drken1215.hatenablog.com/entry/2018/06/08/210000
    //
    // 使用可能場面
    //   * 1 ≤ k ≤ n ≤ 10^7
    //   * pは素数 かつ p > n
    pub fn combination(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        self.fac[n] * (self.finv[k] * self.finv[n - k] % self.modulo) % self.modulo
    }

    // 重複組合せ
    // n種類のものから重複を許してk個選ぶ場合の数: nHk = (n+k-1)Ck
    pub fn homogeneous(&self, n: usize, k: usize) -> usize {
        self.combination(n + k - 1, k)
    }

    // 参考：https://algo-logic.info/combination/
    // 計算量：O(k)
    //
    // 使用可能場面
    //   * n が巨大; 1 ≤ n ≤ 10^9
    //   * k がループ可; 1 ≤ k ≤ 10^5
    //   * pは素数 かつ p > n
    pub fn large_n_combination(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        let mut res = 1;
        for i in (n - k + 1)..=n {
            res = res * i % self.modulo;
        }
        res * self.finv[k] % self.modulo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_combination() {
        let modulo = 1_000_000_007;
        let cap = 1001;
        let comb = ModComb::new(cap, modulo);

        // Test initialization
        assert_eq!(comb.modulo, modulo);
        assert_eq!(comb.fac[0], 1);
        assert_eq!(comb.finv[0], 1);
        assert_eq!(comb.fac.len(), cap);
        assert_eq!(comb.finv.len(), cap);

        // Test combinations
        assert_eq!(comb.combination(5, 3), 10);
        assert_eq!(comb.combination(10, 7), 120);
        assert_eq!(comb.combination(6, 6), 1);
        assert_eq!(comb.combination(1000, 500), 159835829);
    }

    #[test]
    fn test_mod_homogeneous() {
        let modulo = 1_000_000_007;
        let cap = 1001;
        let comb = ModComb::new(cap, modulo);

        // Test homogeneous combinations
        assert_eq!(comb.homogeneous(5, 3), 35); // 5H3 = 7C3 = 35
        assert_eq!(comb.homogeneous(10, 7), 11440); // 10H7 = 16C7 = 11440
        assert_eq!(comb.homogeneous(6, 0), 1); // 6H0 = 5C0 = 1
        assert_eq!(comb.homogeneous(10, 2), 55); // 10H2 = 11C2 = 55
    }

    #[test]
    fn test_large_n_combination() {
        let modulo = 1_000_000_007;
        let cap = 200_001; // 1 <= k <= 2 * 10^5
        let comb = ModComb::new(cap, modulo);

        // This is the same as the regular combination test, but using large_n_combination.
        assert_eq!(comb.large_n_combination(5, 3), 10);
        assert_eq!(comb.large_n_combination(10, 7), 120);
        assert_eq!(comb.large_n_combination(6, 6), 1);

        // Test for cases where n is very large
        let large_n = 1_000_000_000;
        assert_eq!(comb.large_n_combination(large_n, 141421), 516595147);
        assert_eq!(comb.large_n_combination(large_n, 173205), 589953354);
    }
}
