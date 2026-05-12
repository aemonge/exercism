pub fn abbreviate(phrase: &str) -> String {
    let mut r = String::new();
    for word in phrase
        .replace("-", " ")
        .replace("_", " ")
        .split_whitespace()
    {
        let mut it = word.chars();

        let c: char = it
            .next()
            .unwrap_or(' ');

        r.push(c.to_ascii_uppercase());

        for c in it {
            if c.is_ascii_uppercase() {
                r.push(c);
            }
        }
    }
    dbg!(&phrase);
    dbg!(&r);
    r
}
