import gleam/int

pub fn convert(number: Int) -> String {
  {
    case number % 3 {
      0 -> "Pling"
      _ -> ""
    }
    <> case number % 5 {
      0 -> "Plang"
      _ -> ""
    }
    <> case number % 7 {
      0 -> "Plong"
      _ -> ""
    }
  }
  |> fn(x: String) {
    case x {
      "" -> int.to_string(number)
      _ -> x
    }
  }
}
