pub fn abbreviate(phrase: &str) -> String {
    let mut r = String::new();
    for word in phrase
        .replace("-", " ")
        .replace("_", " ")
        .split_whitespace()
    {
        let mut it = word.chars();
        let mut all_caps_lock = true;

        let c: char = it
            .next()
            .unwrap_or(' ');

        all_caps_lock = all_caps_lock && c.is_ascii_uppercase();
        r.push(c.to_ascii_uppercase());

        for c in it {
            all_caps_lock = all_caps_lock && c.is_ascii_uppercase();
            if c.is_ascii_uppercase() && !all_caps_lock {
                r.push(c);
            }
        }
    }
    dbg!(&phrase);
    dbg!(&r);
    r
}
