pub fn series(digits: &str, k: usize) -> Vec<String> {
    let len = digits.len();
    if k > len {
        vec![]
    } else {
        let mut r = Vec::new();
        let n = len - k + 1;

        for i in 0..n {
            let v = &digits[i..i + k];
            r.push(v.to_string());
        }
        r
    }
}
