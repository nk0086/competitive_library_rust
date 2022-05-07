use cargo_snippet::snippet;

#[snippet]
pub fn combination(n: usize, k: usize, m: usize) -> usize {
    let r = k.min(n - k);
    let mut ret = 1;

    for i in 0..r {
        ret = ret * (n - i) % m * modpow(i + 1, m - 2, m) % m;
    }

    ret
}

#[snippet("combination")]
fn modpow(mut base: usize, mut exp: usize, n: usize) -> usize {
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
    use super::combination;

    #[test]
    fn test_combination() {
        assert_eq!(combination(4, 2, 5), 1);
    }
}
