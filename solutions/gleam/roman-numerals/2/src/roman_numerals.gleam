fn simbolize(
  n number: Int,
  sss next_next_symbol,
  ss next_symbol: String,
  s symbol: String,
) -> String {
  case number {
    1 -> symbol
    2 -> symbol <> symbol
    3 -> symbol <> symbol <> symbol
    4 -> symbol <> next_symbol
    5 -> next_symbol
    6 -> next_symbol <> symbol
    7 -> next_symbol <> symbol <> symbol
    8 -> next_symbol <> symbol <> symbol <> symbol
    9 -> symbol <> next_next_symbol
    _ -> ""
  }
}

pub fn convert(number n: Int) -> String {
  case n {
    _ if n > 10_000 -> {
      simbolize(n, "M", "D", "C") <> convert(number: { n / 10 })
    }
    _ if n > 1000 -> {
      simbolize(n, "D", "C", "L") <> convert(number: { n / 10 })
    }
    _ if n > 100 -> {
      simbolize(n, "C", "L", "X") <> convert(number: { n / 10 })
    }
    _ if n > 10 -> {
      simbolize(n, "L", "X", "V") <> convert(number: { n / 10 })
    }
    _ -> {
      simbolize(n, "X", "V", "I")
    }
  }
}
