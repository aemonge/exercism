pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];

    let mut m = n;
    let mut i = 2;
    while m > 1 {
        if m % i == 0 {
            factors.push(i);
            m = m / i;
        } else {
            i += 1;
        }
    }

    factors
}
