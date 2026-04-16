fn list_primes(limit: u32) -> Vec<u32> {
    // See: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
    let mut is_prime = vec![true; (limit + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2;
    while p * p <= limit {
        if is_prime[p as usize] {
            // For this prime p, eliminate numbers that are divisible by p
            // (except p itself).
            // Classical sieve generates multiples p^2, p^2 + p, ...;
            for x in 2..=limit {
                is_prime[x as usize] &= x % p != 0 || x == p;
            }
        }
        p += 1;
    }

    // Collect all i in [2, limit] that survived as prime.
    let mut primes: Vec<u32> = (2..=limit).collect();
    primes.retain(|&x| is_prime[x as usize]);

    primes
}

pub fn nth(i: u32) -> u32 {
    if i == 0 {
        return 2;
    }

    let n = i + 1;

    // Prime Number Theorem (PNT):
    //   π(x) ~ x / ln x, so the n-th prime p_n ~ n ln n.
    // See: https://en.wikipedia.org/wiki/Prime_number_theorem
    let limit = if n < 6 {
        15 // small hand-tuned limit so that first few primes (2,3,5,7,11,13) fit
    } else {
        let nf = n as f64;
        (nf * (nf.ln()
            + nf.ln()
                .ln()))
        .ceil() as u32
    };

    let list = list_primes(limit);
    list[i as usize]
}
