use cargo_snippet::snippet;

/// 素数判定（素朴）
///
/// `2..=√n` の範囲で割り切れるかだけ確認すればよい。
///
/// ## 計算量
/// - `O(√n)`
#[snippet]
pub fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// エラトステネスのふるい
///
/// `1..=n` の素数を列挙して返す。
///
/// ## 計算量
/// - `O(n log log n)`（典型）
#[snippet]
pub fn eratosthenes_sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }

    let mut primes = vec![];
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
            let mut j = i * 2;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    primes
}

/// 素因数分解（素数リスト版）
///
/// `primes`（昇順の素数列）でのみ試し割りする。
/// `p*p > n` で打ち切り、最後に `n!=1` ならそれが残りの素因数。
///
/// 典型: `primes = eratosthenes_sieve(√maxA)` を1回作って使い回す。
///
/// ## 計算量
/// - 目安: `O(π(√n) + ω(n))`
///   - `π(x)` は **x 以下の素数の個数**（prime counting function）
///   - `ω(n)` は **n の異なる素因数の個数**
#[snippet]
pub fn factorize_with_primes(mut n: usize, primes: &[usize]) -> Vec<(usize, usize)> {
    debug_assert!(n >= 1);
    let mut factors: Vec<(usize, usize)> = Vec::new();

    for &p in primes {
        if p * p > n {
            break;
        }
        if n % p != 0 {
            continue;
        }

        let mut count = 0;
        while n % p == 0 {
            n /= p;
            count += 1;
        }
        factors.push((p, count));
    }

    if n != 1 {
        factors.push((n, 1));
    }

    factors
}

/// 素因数分解（素朴版）
///
/// `2..=√n` を全探索して試し割りする。
/// 実装は簡単だが、入力が大きい＆回数が多いと重くなりやすい。
///
/// ## 計算量
/// - `O(√n)`
#[snippet]
pub fn prime_factors(mut n: usize) -> Vec<(usize, usize)> {
    let mut factors: Vec<(usize, usize)> = Vec::new();

    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i != 0 {
            continue;
        }

        let mut count = 0;
        while n % i == 0 {
            count += 1;
            n /= i;
        }

        factors.push((i, count));
    }

    if n != 1 {
        factors.push((n, 1));
    }

    factors
}

/// 約数の列挙
///
/// `i` と `n/i` をペアで拾う。最後にソートして返す。
///
/// ## 計算量
/// - 列挙: `O(√n)`
/// - ソート: `O(d(n) log d(n))`（`d(n)` は約数個数）
#[snippet]
pub fn divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();

    for i in 1..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }

    divisors.sort();
    divisors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(1_000_000_007));
        assert!(!is_prime(1_000_000_003));
        assert!(!is_prime(1));
    }

    #[test]
    fn test_eratosthenes_sieve() {
        assert_eq!(eratosthenes_sieve(1), vec![]);
        assert_eq!(eratosthenes_sieve(2), vec![2]);
        assert_eq!(eratosthenes_sieve(10), vec![2, 3, 5, 7]);
        assert_eq!(eratosthenes_sieve(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
        assert_eq!(
            eratosthenes_sieve(30),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(60), vec![(2, 2), (3, 1), (5, 1)]);
        assert_eq!(prime_factors(84), vec![(2, 2), (3, 1), (7, 1)]);
        assert_eq!(prime_factors(101), vec![(101, 1)]);
    }

    #[test]
    fn test_factorize_with_primes() {
        let primes = eratosthenes_sieve(1_000);
        assert_eq!(
            factorize_with_primes(60, &primes),
            vec![(2, 2), (3, 1), (5, 1)]
        );
        assert_eq!(
            factorize_with_primes(84, &primes),
            vec![(2, 2), (3, 1), (7, 1)]
        );
        assert_eq!(factorize_with_primes(101, &primes), vec![(101, 1)]);
    }

    #[test]
    fn test_divisors() {
        assert_eq!(divisors(60), vec![1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]);
        assert_eq!(divisors(84), vec![1, 2, 3, 4, 6, 7, 12, 14, 21, 28, 42, 84]);
        assert_eq!(divisors(101), vec![1, 101]);
    }
}
