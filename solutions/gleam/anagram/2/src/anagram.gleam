import gleam/io
import gleam/string

fn emove(from from: String, l l: String, acc acc: String) -> String {
  case string.to_graphemes(from) {
    [] -> acc
    [le, ..rest] -> {
      case le == l {
        True -> emove(string.join(rest, ""), "", acc)
        False -> emove(string.join(rest, ""), l, { le <> acc })
      }
    }
  }
}

fn remove(from from: String, l l: String) -> String {
  string.reverse(emove(from, l, ""))
}

pub fn contains_all_letters(
  from from: String,
  posible_anagram posible_anagram: String,
  acc acc: Bool,
) -> Bool {
  case string.to_graphemes(from) {
    [first_letter, ..rest_word] -> {
      io.debug(remove("hola", "h"))
      io.debug(remove("hola", "o"))
      io.debug("--")
      io.debug(posible_anagram)
      io.debug(first_letter)
      let croped = string.crop(posible_anagram, first_letter)
      io.debug(croped)
      io.debug("-----------")
      let has_cropped = croped != posible_anagram
      contains_all_letters(string.join(rest_word, ""), croped, {
        acc && has_cropped
      })
    }
    [] -> acc
  }
}

fn is_anagram(word: String, posible_anagram: String) -> Bool {
  let posible_anagram = string.lowercase(posible_anagram)
  let word = string.lowercase(word)
  let equal = posible_anagram == word

  case equal {
    True -> False
    _ -> contains_all_letters(word, posible_anagram, True)
  }
}

fn find_anagrams_(
  word: String,
  candidates: List(String),
  acc: List(String),
) -> List(String) {
  case candidates {
    [w, ..rest] -> {
      case is_anagram(word, w) {
        True -> find_anagrams_(word, rest, [w, ..acc])
        False -> find_anagrams_(word, rest, acc)
      }
    }
    [] -> acc
  }
}

pub fn find_anagrams(word: String, candidates: List(String)) -> List(String) {
  find_anagrams_(word, candidates, [])
}
