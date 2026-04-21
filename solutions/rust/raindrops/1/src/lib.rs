pub fn raindrops(n: u32) -> String {
    let mut r = String::new();
    let mut f = false;

    if n.is_multiple_of(3) {
        r = "Pling".to_string();
        f |= true;
    }
    if n.is_multiple_of(5) {
        r = format!("{}Plang", r);
        f |= true;
    }
    if n.is_multiple_of(7) {
        r = format!("{}Plong", r);
        f |= true;
    }
    if f { r } else { n.to_string() }
}
