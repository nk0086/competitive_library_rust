use cargo_snippet::snippet;

#[snippet]
pub fn combination_mod(number: usize, n: usize) -> (Vec<usize>, Vec<usize>) {
    let mut numerator = vec![1; number];
    let mut denominator = vec![1; number];

    for i in 1..number {
        numerator[i] = (numerator[i - 1] * (i + 1)) % n;
        denominator[i] = modpow(numerator[i], n - 2, n);
    }
    (numerator, denominator)
}

#[snippet("combination_mod")]
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
    use super::combination_mod;

    #[test]
    fn test_frac() {
        todo!();
    }
}
