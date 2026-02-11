import gleam/io

const space = "."

fn prev_char(letter: String) -> String {
  case letter {
    "B" -> "A"
    "C" -> "B"
    "D" -> "C"
    "E" -> "D"
    "F" -> "E"
    "G" -> "F"
    "H" -> "G"
    "I" -> "H"
    "J" -> "I"
    "K" -> "J"
    "L" -> "K"
    "M" -> "L"
    "N" -> "M"
    "O" -> "N"
    "P" -> "O"
    "Q" -> "P"
    "R" -> "Q"
    "S" -> "R"
    "T" -> "S"
    "U" -> "T"
    "V" -> "U"
    "W" -> "V"
    "X" -> "W"
    "Y" -> "X"
    "Z" -> "Y"
    "A" -> "Z"
    _ -> letter
  }
}

fn next_char(letter: String) -> String {
  case letter {
    "A" -> "B"
    "B" -> "C"
    "C" -> "D"
    "D" -> "E"
    "E" -> "F"
    "F" -> "G"
    "G" -> "H"
    "H" -> "I"
    "I" -> "J"
    "J" -> "K"
    "K" -> "L"
    "L" -> "M"
    "M" -> "N"
    "N" -> "O"
    "O" -> "P"
    "P" -> "Q"
    "Q" -> "R"
    "R" -> "S"
    "S" -> "T"
    "T" -> "U"
    "U" -> "V"
    "V" -> "W"
    "W" -> "X"
    "X" -> "Y"
    "Y" -> "Z"
    "Z" -> "A"
    _ -> letter
  }
}

fn odd_line(current_letter: String, target_letter: String) -> String {
  case current_letter == target_letter {
    True -> current_letter
    False ->
      space <> odd_line(next_char(current_letter), target_letter) <> space
  }
}

fn even_line(current_letter: String, target_letter: String) -> String {
  case current_letter == target_letter {
    True -> current_letter
    False ->
      even_line(next_char(current_letter), target_letter)
      <> space
      <> even_line(next_char(current_letter), target_letter)
  }
}

pub fn build_rec(
  current_letter: String,
  target_letter: String,
  is_odd_line: Bool,
) -> String {
  let same_letter = current_letter == target_letter

  case same_letter, is_odd_line {
    True, True -> ""
    True, False -> current_letter
    False, True ->
      odd_line(current_letter, target_letter)
      <> "\n"
      <> build_rec(next_char(current_letter), target_letter, { !is_odd_line })
    False, False ->
      even_line(current_letter, target_letter)
      <> "\n"
      <> build_rec(next_char(current_letter), target_letter, { !is_odd_line })
  }
}

pub fn build_inv_rec(
  current_letter: String,
  target_letter: String,
  is_odd_line: Bool,
) -> String {
  let same_letter = current_letter == target_letter

  case same_letter, is_odd_line {
    True, True -> ""
    True, False -> current_letter
    False, True ->
      odd_line(current_letter, target_letter)
      <> "\n"
      <> build_inv_rec(prev_char(current_letter), target_letter, {
        !is_odd_line
      })
    False, False ->
      even_line(current_letter, target_letter)
      <> "\n"
      <> build_inv_rec(prev_char(current_letter), target_letter, {
        !is_odd_line
      })
  }
}

pub fn build(letter: String) -> String {
  // build_rec("A", letter, False)
  build_inv_rec(letter, "A", False)
  // build_rec("A", letter, False) <> "\n" <> build_inv_rec("A", letter, False)
}
