#[derive(PartialEq)]
enum Token {
    Brace,
    Bracket,
    Parenthesis,
}

pub fn brackets_are_balanced(string: &str) -> bool {
    dbg!(string);
    let mut open_count: isize = 0;
    let mut open_stack: Vec<Token> = vec![];

    for c in string.chars() {
        if c == '{' || c == '[' || c == '(' {
            open_count += 1;
        }

        match c {
            '{' => {
                open_stack.push(Token::Brace);
            }
            '[' => {
                open_stack.push(Token::Bracket);
            }
            '(' => {
                open_stack.push(Token::Parenthesis);
            }

            '}' if open_stack
                .last()
                .is_none_or(|v| *v != Token::Brace) =>
            {
                return false;
            }

            ']' if open_stack
                .last()
                .is_none_or(|v| *v != Token::Bracket) =>
            {
                return false;
            }

            ')' if open_stack
                .last()
                .is_none_or(|v| *v != Token::Parenthesis) =>
            {
                return false;
            }
            _ => (),
        }

        if c == '}' || c == ']' || c == ')' {
            open_count -= 1;
            open_stack.pop();
        }
    }
    open_count == 0
}
