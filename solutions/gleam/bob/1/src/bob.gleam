import gleam/string

pub fn hey(remark: String) -> String {
  let kramer = string.reverse(remark) |> string.trim
  let remark = string.trim(remark)

  let is_shouting = string.lowercase(remark) != string.uppercase(remark)
  let is_shouting = is_shouting && remark == string.uppercase(remark)

  case remark, kramer, is_shouting {
    "", "", _ -> "Fine. Be that way!"
    _, "?" <> _, False -> "Sure."
    _, "?" <> _, True -> "Calm down, I know what I'm doing!"
    _, _, True -> "Whoa, chill out!"
    _, _, _ -> "Whatever."
  }
}
