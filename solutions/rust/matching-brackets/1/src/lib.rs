pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: isize = 0;
    let mut braces: isize = 0;
    let mut parentheses: isize = 0;

    for c in string.chars() {
        match c {
            '{' => braces += 1,
            '}' => braces -= 1,
            '[' => brackets += 1,
            ']' => brackets -= 1,
            '(' => parentheses += 1,
            ')' => parentheses -= 1,
            _ => (),
        }

        // If any is negative, it means  }{
        if brackets < 0 || braces < 0 || parentheses < 0 {
            return false;
        }

        // If any is bigger or equal to two and the are are not 0
        // it means {{] a miss-match
        // AKA: If any is two, all must be zero
        if (brackets >= 2 || braces >= 2 || parentheses >= 2)
            && (brackets != 0 || braces != 0 || parentheses != 0)
        {
            return false;
        }
    }
    brackets == 0 && braces == 0 && parentheses == 0
}
