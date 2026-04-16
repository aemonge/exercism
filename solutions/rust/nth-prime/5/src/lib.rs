fn list_primes(n: u32) -> Vec<u32> {
    let prime_ceil = n * n;
    let mut candidates: Vec<u32> = (2..prime_ceil).collect();

    let mut len: u32 = candidates.len() as u32;
    let mut i: u32 = 2;
    while i <= len {
        candidates.retain(|&x| (x == i) || (x % i != 0));

        len = candidates.len() as u32;
        i += 1;
    }

    candidates
}

pub fn nth(i: u32) -> u32 {
    let n = if i < 2 { 2 } else { i + 1 };
    let list = list_primes(n);

    list[i as usize]
}
