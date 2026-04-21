pub fn build_proverb(list: &[&str]) -> String {
    if list.len() <= 0 {
        String::new()
    } else {
        let mut r = String::new();
        let mut first: &str;
        let mut second: &str;
        let len = list.len();

        for i in 1..len {
            first = list[i - 1];
            second = list[i];
            r = format!("{}For want of a {} the {} was lost.\n", r, first, second);
        }
        r = format!("{}And all for the want of a {}.", r, list[0]);
        r
    }
}
