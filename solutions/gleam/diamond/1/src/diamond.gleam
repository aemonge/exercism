import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string

const space = "."

const ord_a = 65

fn ord_from_a(letter: String) -> Int {
  let letter = string.uppercase(letter)
  let assert [codepoint, ..] = string.to_utf_codepoints(letter)
  let ord_letter = string.utf_codepoint_to_int(codepoint)
  ord_letter - ord_a
}

fn next_char(letter: String) -> String {
  case string.uppercase(letter) {
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

fn odd_line(letter: String) -> String {
  let spaces = list.repeat(space, ord_from_a(letter))
  let spaces = string.join(spaces, "")

  spaces <> letter <> spaces
}

fn even_line(letter: String) -> String {
  let spaces = list.repeat(space, ord_from_a(letter))
  let spaces = string.join(spaces, "")

  letter <> spaces <> letter
}

fn building(target: String, current: String) -> String {
  case target == current {
    True -> ""
    False -> building(target, next_char(current))
  }
}

pub fn build(letter: String) -> String {
  building(letter, "A")
  // let lines = ord_from_a(letter)
  // case lines {
  //   0 -> letter
  //   2 -> even_line(letter) <> "\n" <> build(next_char(letter))
  //   _ -> odd_line(letter) <> "\n" <> build(next_char(letter))
  // }
}
