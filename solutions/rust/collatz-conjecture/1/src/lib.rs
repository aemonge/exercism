fn collatz_(n: u64, i: u64, acc: u64) -> Option<u64> {
    if i > n || i == 0 {
        None
    } else if i == 1 {
        Some(acc)
    } else {
        collatz_(
            n,
            if i.is_multiple_of(2) {
                i / 2
            } else {
                i * 3 + 1
            },
            acc + 1,
        )
    }
}
pub fn collatz(n: u64) -> Option<u64> {
    collatz_(n * 2, n, 0)
}
