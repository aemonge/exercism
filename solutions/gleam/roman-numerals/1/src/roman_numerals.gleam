fn simbolize(n number: Int, ss next_symbol: String, s symbol: String) -> String {
  case number {
    1 -> symbol
    2 -> symbol <> symbol
    3 -> symbol <> symbol <> symbol
    4 -> symbol <> next_symbol
    5 -> next_symbol
    _ -> ""
  }
}

pub fn convert(number: Int) -> String {
  let nine_hund = number / 900
  let four_hund = number / 400
  let nine_ten = number / 90
  let four_ten = number / 40
  let nine = number / 9
  let four = number / 4

  case number, nine_hund, four_hund, nine_ten, four_ten, nine, four {
    number, nine_hund, _, _, _, _, _ if nine_hund > 0 -> {
      let number = number % 900
      "CM" <> convert(number)
    }
    number, _, four_hund, _, _, _, _ if four_hund > 0 -> {
      let number = number % 400
      "CD" <> convert(number)
    }
    number, _, _, nine_ten, _, _, _ if nine_ten > 0 -> {
      let number = number % 90
      "XC" <> convert(number)
    }
    number, _, _, _, four_ten, _, _ if four_ten > 0 -> {
      let number = number % 40
      "XL" <> convert(number)
    }
    number, _, _, _, _, nine, _ if nine > 0 -> {
      let number = number % 9
      "IX" <> convert(number)
    }
    number, _, _, _, _, _, four if four > 0 -> {
      let number = number % 4
      "IV" <> convert(number)
    }
    number, _, _, _, _, _, _ -> {
      let #(thousands, number) = #(number / 1000, number % 1000)
      let #(five_hundreds, number) = #(number / 500, number % 500)
      let #(hundreds, number) = #(number / 100, number % 100)
      let #(fifties, number) = #(number / 50, number % 50)
      let #(tens, number) = #(number / 10, number % 10)
      let #(fives, ones) = #(number / 5, number % 5)

      simbolize(thousands, "O", "M")
      <> simbolize(five_hundreds, "M", "D")
      <> simbolize(hundreds, "D", "C")
      <> simbolize(fifties, "C", "L")
      <> simbolize(tens, "L", "X")
      <> simbolize(fives, "X", "V")
      <> simbolize(ones, "V", "I")
    }
  }
}
