use cargo_snippet::snippet;

// 素数判定
// 2 ~ sqrt(n)の整数で割れるかだけを見ればよい
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

// エラトステネスのふるい
// 1 以上 N 以下の整数が素数かどうかを返す
#[snippet]
pub fn eratosthenes_sieve(n: usize) -> Vec<usize> {
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

// 素因数分解
// 小さい数字から割り続けていくことがミソ
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

// 約数の列挙
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
        let primes_1 = vec![];
        let primes_2 = vec![2];
        let primes_10 = vec![2, 3, 5, 7];
        let primes_20 = vec![2, 3, 5, 7, 11, 13, 17, 19];
        let primes_30 = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

        assert_eq!(eratosthenes_sieve(1), primes_1);
        assert_eq!(eratosthenes_sieve(2), primes_2);
        assert_eq!(eratosthenes_sieve(10), primes_10);
        assert_eq!(eratosthenes_sieve(20), primes_20);
        assert_eq!(eratosthenes_sieve(30), primes_30);
    }

    #[test]
    fn test_prime_factors() {
        let n1 = 60;
        let result1 = prime_factors(n1);
        assert_eq!(result1, vec![(2, 2), (3, 1), (5, 1)]);

        let n2 = 84;
        let result2 = prime_factors(n2);
        assert_eq!(result2, vec![(2, 2), (3, 1), (7, 1)]);

        let n3 = 101;
        let result3 = prime_factors(n3);
        assert_eq!(result3, vec![(101, 1)]);
    }

    #[test]
    fn test_divisors() {
        let n1 = 60;
        let result1 = divisors(n1);
        assert_eq!(result1, vec![1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]);

        let n2 = 84;
        let result2 = divisors(n2);
        assert_eq!(result2, vec![1, 2, 3, 4, 6, 7, 12, 14, 21, 28, 42, 84]);

        let n3 = 101;
        let result3 = divisors(n3);
        assert_eq!(result3, vec![1, 101]);
    }
}
