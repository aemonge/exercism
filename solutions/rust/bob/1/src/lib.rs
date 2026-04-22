pub fn reply(message: &str) -> &str {
    let all_caps = message
        .to_string()
        .trim()
        .to_ascii_uppercase()
        == message
        && message
            .to_string()
            .trim()
            .to_ascii_lowercase()
            != message;

    let is_silent = message
        .to_string()
        .trim()
        .is_empty();

    let is_question = message
        .to_string()
        .trim()
        .ends_with("?");

    // ---------------------------------------------------------------------- /

    if is_question {
        if all_caps {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if is_silent {
        "Fine. Be that way!"
    } else if all_caps {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
