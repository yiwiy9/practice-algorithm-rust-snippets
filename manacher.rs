use cargo_snippet::snippet;

/// 文字列アルゴリズム: Manacher 法
/// 文字列が与えられた時、各 i について「文字 i を中心とする最長の回文の半径」を記録した配列 R を O(|S|) で構築するアルゴリズムです。
/// 半径というのは、(全長+1)/2です。
/// ちなみに、普通の Manacher をやると奇数長の回文しか検出できませんが、「a$b$a$a$b」みたいにダミー文字を間に挟むと偶数長のものも検出できるようにできます。
/// https://snuke.hatenablog.com/entry/2014/12/02/235837
#[snippet]
#[snippet("manacher_with_dummy")]
pub fn manacher(s: &Vec<char>) -> Vec<usize> {
    let n = s.len();
    let mut r = vec![0; n];
    let mut i = 0;
    let mut j = 0;
    while i < n {
        while i >= j && i + j < n && s[i - j] == s[i + j] {
            j += 1;
        }
        r[i] = j;
        let mut k = 1;
        while i >= k && k + r[i - k] < j {
            r[i + k] = r[i - k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    r
}

/// ダミー文字を挿入して偶数長の回文にも対応する Manacher 法
/// 戻り値は、ダミー挿入後の文字列に対する回文半径配列（中心 i に対して r[i]）
#[snippet]
pub fn manacher_with_dummy(s: &Vec<char>) -> Vec<usize> {
    // 文字の間にダミー文字 '$' を挿入
    let mut t = Vec::with_capacity(s.len() * 2 + 1);
    t.push('$');
    for &c in s {
        t.push(c);
        t.push('$');
    }

    manacher(&t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manacher_basic_odd_only() {
        let s: Vec<char> = "aba".chars().collect();
        let r = manacher(&s);
        // "aba" → 中央 'b' を中心とする回文長3 → 半径2
        // 両端は 'a' → 半径1
        assert_eq!(r, vec![1, 2, 1]);
    }

    #[test]
    fn test_manacher_no_palindrome() {
        let s: Vec<char> = "abc".chars().collect();
        let r = manacher(&s);
        // 各文字単体しか回文にならない → 半径1ずつ
        assert_eq!(r, vec![1, 1, 1]);
    }

    #[test]
    fn test_manacher_with_dummy_odd() {
        let s: Vec<char> = "aba".chars().collect();
        let r = manacher_with_dummy(&s);
        // ダミー付き: $a$b$a$
        // → r = [1,2,1,4,1,2,1]
        assert_eq!(r, vec![1, 2, 1, 4, 1, 2, 1]);
    }

    #[test]
    fn test_manacher_with_dummy_even() {
        let s: Vec<char> = "abba".chars().collect();
        let r = manacher_with_dummy(&s);
        // ダミー付き: $a$b$b$a$
        // → r = [1,2,1,2,5,2,1,2,1]
        assert_eq!(r, vec![1, 2, 1, 2, 5, 2, 1, 2, 1]);
    }

    #[test]
    fn test_manacher_with_dummy_no_palindrome() {
        let s: Vec<char> = "abc".chars().collect();
        let r = manacher_with_dummy(&s);
        // ダミー付き: $a$b$c$
        // → 各位置での半径: 1,2,1,2,1,2,1
        assert_eq!(r, vec![1, 2, 1, 2, 1, 2, 1]);
    }

    #[test]
    fn test_manacher_with_dummy_all_same() {
        let s: Vec<char> = "aaaaa".chars().collect();
        let r = manacher_with_dummy(&s);
        // ダミー付き: $a$a$a$a$a$
        // → r = 対称な値が並ぶ。中心は最大。
        assert_eq!(r, vec![1, 2, 3, 4, 5, 6, 5, 4, 3, 2, 1]);
    }
}
