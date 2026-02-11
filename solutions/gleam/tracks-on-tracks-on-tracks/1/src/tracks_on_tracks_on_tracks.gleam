pub fn new_list() -> List(String) {
  []
}

pub fn existing_list() -> List(String) {
  ["Gleam", "Go", "TypeScript"]
}

pub fn add_language(languages: List(String), language: String) -> List(String) {
  [language, ..languages]
}

fn counting(l: List(String), acc: Int) -> Int {
  case l {
    [] -> acc
    [_, ..rest] -> counting(rest, { acc + 1 })
  }
}

pub fn count_languages(languages: List(String)) -> Int {
  counting(languages, 0)
}

fn reversing(l: List(String), acc: List(String)) -> List(String) {
  case l {
    [] -> acc
    [x, ..rest] -> reversing(rest, [x, ..acc])
  }
}

pub fn reverse_list(languages: List(String)) -> List(String) {
  reversing(languages, [])
}

pub fn exciting_list(languages: List(String)) -> Bool {
  case languages {
    ["Gleam", ..] -> True
    [_, "Gleam"] -> True
    [_, "Gleam", _] -> True
    _ -> False
  }
}
