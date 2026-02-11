pub fn is_armstrong_number(num: u32) -> bool {
    let num_as_chars = num.to_string();
    let len = num_as_chars.len();
    num_as_chars
        .chars()
        .fold(0, |acc, x| {
            acc + x
                .to_digit(10)
                .unwrap_or(0)
                .pow(len as u32)
        })
        == num
}
