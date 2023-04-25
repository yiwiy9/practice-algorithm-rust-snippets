use cargo_snippet::snippet;

// 繰り返し二乗法
#[snippet]
pub fn mod_pow(base: usize, exp: usize, modulo: usize) -> usize {
    if exp == 0 {
        return 1;
    }
    let mut result = mod_pow(base * base % modulo, exp / 2, modulo);
    if exp % 2 == 1 {
        result *= base;
        result %= modulo;
    }
    result
}

// 逆元 num^(modulo-2) : mod modulo
// a*a^(p-2)≡1 : mod p
#[snippet(include = "mod_pow")]
pub fn mod_inv(num: usize, modulo: usize) -> usize {
    mod_pow(num, modulo - 2, modulo)
}

#[cfg(test)]
mod tests {
    use super::*;
    const MOD: usize = 998244353;

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(10, 0, MOD), 1);
        assert_eq!(mod_pow(10, 3, MOD), 1000);
        assert_eq!(mod_pow(10, 10, MOD), 17556470);
    }

    #[test]
    fn test_mod_inv() {
        assert_eq!(mod_inv(7, 13), 2);
        assert_eq!(mod_inv(2, MOD), 499122177);
    }
}
