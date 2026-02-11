import gleam/string

const space = " "

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

fn char(letter: String) -> Int {
  case letter {
    "A" -> 0
    "B" -> 1
    "C" -> 2
    "D" -> 3
    "E" -> 4
    "F" -> 5
    "G" -> 6
    "H" -> 7
    "I" -> 8
    "J" -> 9
    "K" -> 10
    "L" -> 11
    "M" -> 12
    "N" -> 13
    "O" -> 14
    "P" -> 15
    "Q" -> 15
    "R" -> 16
    "S" -> 17
    "T" -> 18
    "U" -> 19
    "V" -> 20
    "W" -> 22
    "X" -> 23
    "Y" -> 24
    "Z" -> 25
    _ -> -1
  }
}

pub fn line_rec(
  c_let: String,
  idx: Int,
  limit_l: String,
  limit: Int,
  display: String,
) -> String {
  case idx == limit, c_let == limit_l {
    True, False -> space
    True, True -> display
    False, False ->
      space <> line_rec(next_char(c_let), { idx + 1 }, limit_l, limit, display)
    False, True ->
      display
      <> line_rec(next_char(c_let), { idx + 1 }, limit_l, limit, display)
  }
}

pub fn line(from: String, to: String) -> String {
  let line = line_rec(from, 0, to, char(to), from)
  let rev = string.drop_left(string.reverse(line), up_to: 1)
  line <> rev
}

fn build_rec(c_let: String, target: String) -> String {
  case c_let == target {
    True -> line(c_let, target)
    False -> {
      line(c_let, target)
      <> "\n"
      <> build_rec(next_char(c_let), target)
      <> "\n"
      <> line(c_let, target)
    }
  }
}

pub fn build(letter: String) -> String {
  build_rec("A", letter)
}
