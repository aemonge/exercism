pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut r: Vec<u32> = vec![];

    for e in factors {
        if *e > 0 {
            for i in (1..limit).rev() {
                if i % e == 0 {
                    r.push(i);
                }
            }
        }
    }

    r.sort();
    r.dedup();
    r.iter()
        .sum()
}
