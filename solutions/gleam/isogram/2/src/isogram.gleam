import gleam/dict
import gleam/string

fn is(phrase phrase: String, acc acc: Bool, d d: dict.Dict(String, Int)) -> Bool {
  case string.to_graphemes(phrase) {
    [] -> acc
    [l, ..rest] -> {
      let has = dict.has_key(d, l)
      let is_isogram = !has || l == "-" || l == " "
      let d = dict.insert(d, l, 1)
      is(string.join(rest, ""), { acc && is_isogram }, d)
    }
  }
}

pub fn is_isogram(phrase phrase: String) -> Bool {
  is(phrase, True, dict.new())
}
