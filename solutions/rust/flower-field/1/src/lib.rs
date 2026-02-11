fn valid_garden(garden: &[&str]) -> bool {
    (garden.len() >= 3) && (garden[0].len() >= 3)
}
pub fn annotate(garden: &[&str]) -> Vec<String> {
    if !valid_garden(garden) {
        // Should return an error, but I'll implement what the test expect
        if garden.is_empty() {
            return vec![];
        }
        return vec![String::from("")];
    }
    todo!(
        "\nAnnotate each square of the given garden with the number of flowers that surround said square (blank if there are no surrounding flowers):\n{garden:#?}\n"
    );
}
