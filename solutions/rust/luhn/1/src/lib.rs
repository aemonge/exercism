fn _is(code: &str) -> bool {
    (code
        .replace(" ", "")
        .chars()
        .rev()
        .step_by(2)
        .map(|x| {
            let x = x
                .to_digit(10)
                .unwrap_or(0)
                * 2;
            if x > 9 { x - 9 } else { x }
        })
        .sum::<u32>()
        % 10)
        == 0
}
fn is_sin_valid(code: &str) -> bool {
    !_is(code)
}
fn is_credit_card_valid(code: &str) -> bool {
    _is(code)
}
/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let len = code.len();
    if len <= 1 {
        return false;
    }
    let spaces = code
        .chars()
        .fold(0, |acc, x| acc + if x == ' ' { 1 } else { 0 });

    match spaces {
        3 => is_credit_card_valid(code),
        _ => is_sin_valid(code),
    }
}
