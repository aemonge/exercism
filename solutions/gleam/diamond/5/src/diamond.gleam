import gleam/io
import gleam/string

const space = "_"

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

fn line_rec(current: String, target: String) -> String {
  case current == "A" {
    True -> target
    False -> line_rec(prev_char(current), target) <> space
  }
}

pub fn line(letter: String) -> String {
  let reversed =
    line_rec(letter, letter) |> string.reverse |> string.drop_left(up_to: 1)
  line_rec(letter, letter) <> reversed
}

fn build_rec(current: String, target: String) -> String {
  case current == target {
    True -> line(current)
    False -> line(current) <> "\n" <> build_rec(next_char(current), target)
  }
}

pub fn build(letter: String) -> String {
  build_rec("A", letter)
  //  <> string.reverse(build_rec("A", letter))
}
