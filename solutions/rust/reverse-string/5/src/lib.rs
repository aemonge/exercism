use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut a = String::new();
    for c in UnicodeSegmentation::graphemes(input, true).rev() {
        a.push_str(c);
    }
    a
}
