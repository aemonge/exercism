pub fn brackets_are_balanced(string: &str) -> bool {
    let mut opener: char = '\0';

    for c in string.chars() {
        match c {
            '{' => {
                if opener != '\0' {
                    return false;
                }
                opener = '{';
            }
            '}' => {
                if opener != '{' {
                    return false;
                }
                opener = '\0';
            }
            '[' => {
                if opener != '\0' {
                    return false;
                }
                opener = '[';
            }
            ']' => {
                if opener != '[' {
                    return false;
                }
                opener = '\0';
            }
            '(' => {
                if opener != '\0' {
                    return false;
                }
                opener = '(';
            }
            ')' => {
                if opener != '(' {
                    return false;
                }
                opener = '\0';
            }
            _ => (),
        }
    }
    true
}
