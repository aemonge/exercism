pub fn series(digits: &str, k: usize) -> Vec<String> {
    let len = digits.len();
    if k > len {
        vec![]
    } else {
        let mut r = Vec::new();

        for i in 0..=len - k {
            let v = &digits[i..i + k];
            r.push(v.to_string());
        }
        r
    }
}
