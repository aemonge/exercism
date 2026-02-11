pub fn reply(guess: Int) -> String {
  case guess {
    i if i > 43 -> "Too high"
    43 -> "So close"
    42 -> "Correct"
    41 -> "So close"
    i if i < 41 -> "Too low"
    _ -> "Invalid guess"
  }
}
