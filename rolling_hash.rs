use cargo_snippet::snippet;

#[snippet("RollingHash")]
/// ローリングハッシュ（Rolling Hash）
/// - 基礎: https://koseki2580.github.io/study-docs/docs/Algorithm/rolling-hash/
/// - 実践: https://qiita.com/keymoon/items/11fac5627672a6d6a9f6
///     - ロリハ(RollingHash)のModを (2^61 - 1) にすると安全で爆速になってむちゃくちゃ幸せになれます。
/// - 入力は &str を受け取り、UTF-8 の「バイト列」をハッシュします（速くて一般的）
/// - 乱択 base（毎回ランダム）で狙い撃ちに強い
/// - get(l, r) は半開区間 [l, r)
#[derive(Debug, Clone)]
pub struct RollingHash {
    base: u64,      // 乗算の基数（乱択で決める）
    pow: Vec<u64>,  // pow[i] = base^i (mod MOD)
    hash: Vec<u64>, // hash[i] = 先頭から i バイトのハッシュ（hash[0] = 0）
}

#[snippet("RollingHash")]
impl RollingHash {
    /// メルセンヌ素数 2^61 - 1
    pub const MOD: u64 = (1u64 << 61) - 1;

    /// ランダム（乱択）base で構築（実戦は基本これを使う）
    pub fn new(s: &str) -> Self {
        let base = Self::random_base();
        Self::with_base(s, base)
    }

    /// base を明示して構築（テスト・再現用）
    pub fn with_base(s: &str, base: u64) -> Self {
        assert!((2..Self::MOD).contains(&base), "invalid base");
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut pow = Vec::with_capacity(n + 1);
        let mut hash = Vec::with_capacity(n + 1);
        pow.push(1); // base^0
        hash.push(0); // 空列のハッシュは 0

        // h[i+1] = h[i] * base + bytes[i]   (mod MOD)
        // p[i+1] = p[i] * base              (mod MOD)
        let mut h = 0u64;
        let mut p = 1u64;
        for &b in bytes {
            h = Self::add_mod(Self::mul_mod(h, base), b as u64);
            p = Self::mul_mod(p, base);
            hash.push(h);
            pow.push(p);
        }

        Self { base, pow, hash }
    }

    /// 部分列のハッシュ（半開区間 [l, r)）
    /// 計算式: hash[r] - hash[l] * base^(r-l) (mod MOD)
    #[inline]
    pub fn get(&self, l: usize, r: usize) -> u64 {
        debug_assert!(l <= r && r <= self.len());
        let t = Self::mul_mod(self.hash[l], self.pow[r - l]);
        let mut x = self.hash[r].wrapping_add(Self::MOD).wrapping_sub(t);
        if x >= Self::MOD {
            x -= Self::MOD;
        } // (a - b) mod M の負回避
        x
    }

    /// 元文字列の長さ（バイト長）
    #[inline]
    pub fn len(&self) -> usize {
        self.hash.len() - 1
    }

    /// 元文字列が空かどうか
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// このハッシュで使われた base（デバッグ用）
    #[inline]
    pub fn base(&self) -> u64 {
        self.base
    }

    /// (a + b) mod MOD（a,b < MOD を仮定）
    #[inline]
    fn add_mod(a: u64, b: u64) -> u64 {
        let mut s = a + b;
        if s >= Self::MOD {
            s -= Self::MOD;
        }
        s
    }

    /// (a * b) mod MOD（u128 を使った高速・安全な実装）
    /// 2^61-1 はメルセンヌ素数なので、以下のように 61bit で畳み込むと高速に mod できる:
    ///   x mod MOD = (x_low + x_high) mod MOD  （必要ならもう一回だけ畳み込み）
    #[inline]
    fn mul_mod(a: u64, b: u64) -> u64 {
        Self::reduce((a as u128) * (b as u128))
    }

    /// u128 を 2^61-1 で剰余する（1回の fold で十分）
    #[inline]
    fn reduce(x: u128) -> u64 {
        let low = (x as u64) & Self::MOD; // 下位 61bit
        let high = (x >> 61) as u64; // 上位を 61bit 右シフト
        let mut s = low + high; // low + high が答え候補
        if s >= Self::MOD {
            s -= Self::MOD;
        }
        s
    }

    /// 依存なし・軽量な疑似乱数（xorshift）で base を作る。
    /// - 0 や 1 は避ける
    /// - 小さすぎる値も避ける（1<<30 以上）→ 分布が偏りにくい/衝突しづらい
    /// - 乱択（ランダム化）により、「狙い撃ち衝突」をほぼ不可能にする
    fn random_base() -> u64 {
        // 固定シードで十分（毎回 new() で再度 xorshift するため「乱択的」に振る舞う）
        let mut x = 0x9e3779b97f4a7c15u64; // 黄金比ベースの定数（任意）
                                           // xorshift64
        x ^= x << 7;
        x ^= x >> 9;
        x ^= x << 8;

        // 範囲: [2^30, MOD-1]
        let span = Self::MOD - (1u64 << 30);
        let b = (x % span) + (1u64 << 30);
        if b <= 1 {
            2
        } else {
            b
        } // 念のため 0/1 を回避
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_substring_should_match() {
        let s = "abacaba";
        let base = 911_382_323u64;
        let rh = RollingHash::with_base(s, base);

        // "aba" と "aba"
        let h1 = rh.get(0, 3);
        let h2 = rh.get(4, 7);
        assert_eq!(h1, h2);
    }

    #[test]
    fn different_substring_should_not_match() {
        let s = "abcdefg";
        let base = 972_663_749u64;
        let rh = RollingHash::with_base(s, base);

        let a = rh.get(0, 3); // "abc"
        let b = rh.get(1, 4); // "bcd"
        assert_ne!(a, b);
    }

    #[test]
    fn concatenation_property_holds() {
        // get(l, r) == get(l, m) * base^(r-m) + get(m, r)
        let s = "rollinghash";
        let base = 1_000_000_007u64;
        let rh = RollingHash::with_base(s, base);

        for l in 0..=s.len() {
            for m in l..=s.len() {
                for r in m..=s.len() {
                    let left = rh.get(l, m);
                    let right = rh.get(m, r);
                    let combined =
                        RollingHash::add_mod(RollingHash::mul_mod(left, rh.pow[r - m]), right);
                    assert_eq!(rh.get(l, r), combined);
                }
            }
        }
    }

    #[test]
    fn empty_and_single_char_cases() {
        // 空文字
        let s = "";
        let base = 317u64;
        let rh = RollingHash::with_base(s, base);
        assert!(rh.is_empty());
        assert_eq!(rh.len(), 0);
        assert_eq!(rh.get(0, 0), 0);

        // 1文字
        let s = "X";
        let rh = RollingHash::with_base(s, base);
        assert_eq!(rh.len(), 1);
        assert_eq!(rh.get(0, 1), b'X' as u64);
    }

    #[test]
    fn utf8_should_be_hashed_as_bytes() {
        let s = "aあb"; // 'あ' は3バイト: E3 81 82
        let base = 1234567891u64;
        let rh = RollingHash::with_base(s, base);

        // UTF-8長は5バイト
        assert_eq!(rh.len(), 5);

        // get(1,4) は "あ" の3バイト分
        let sub1 = rh.get(1, 4);
        let sub2 = rh.get(1, 4); // 同じ範囲なら当然一致
        assert_eq!(sub1, sub2);
    }

    #[test]
    fn random_base_is_valid() {
        let s = "example";
        let rh = RollingHash::new(s);
        assert!(rh.base() >= (1u64 << 30) && rh.base() < RollingHash::MOD);
        // get(0,0) == 0, get(0,1) == 最初のバイト
        assert_eq!(rh.get(0, 0), 0);
        assert_eq!(rh.get(0, 1), b'e' as u64);
    }
}
