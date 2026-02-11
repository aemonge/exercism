fn _is(code: Vec<char>) -> bool {
    let mut i = code.len() - 1;
    let mut even = false;
    let mut sum = 0;

    loop {
        let mut val = code[i]
            .to_digit(10)
            .unwrap_or(0);
        if even {
            val *= 2;
            val = if val > 9 { val - 9 } else { val };
        }
        sum += val;
        if i == 0 {
            // Ugly Rust type system
            break;
        }
        i -= 1;
        even = !even;
    }

    sum % 10 == 0
}
/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code: Vec<char> = code
        .replace(" ", "")
        .chars()
        .collect();
    let len = code.len();
    if len <= 1 {
        return false;
    }
    if len
        != code
            .iter()
            .filter(|x| x.is_ascii_digit())
            .count()
    {
        return false;
    }
    _is(code)
}
