pub fn combination_mod(number: usize, MOD: usize) -> (Vec<usize>, Vec<usize>) {
    let mut numerator = vec![1; number];
    let mut denominator = vec![1; number];

    for i in 1..number {
        numerator[i] = (numerator[i - 1] * (i + 1)) % MOD;
        denominator[i] = modpow(numerator[i], MOD - 2, MOD);
    }
    (numerator, denominator)
}

#[cfg(test)]
mod tests {
    use super::combination_mod;

    #[test]
    fn test_frac() {
        todo!();
    }
}
