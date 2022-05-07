use cargo_snippet::snippet;

#[snippet]
pub fn modpow(mut base: usize, mut exp: usize, n: usize) -> usize {
    let mut res = 1;
    while exp > 0 {
        if exp & 1 != 0 {
            res = res * base % n;
        }

        base = base * base % n;
        exp >>= 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::modpow;

    #[test]
    fn test_modpow() {
        assert_eq!(modpow(2, 10, 1000), 24);
        assert_eq!(modpow(3, 5, 5), 3);
    }
}
