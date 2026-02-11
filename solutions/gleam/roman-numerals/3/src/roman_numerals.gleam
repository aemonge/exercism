fn symbolize(number: Int, s: #(String, String, String)) -> String {
  case number {
    1 -> s.0
    2 -> s.0 <> s.0
    3 -> s.0 <> s.0 <> s.0
    4 -> s.0 <> s.1
    5 -> s.1
    6 -> s.1 <> s.0
    7 -> s.1 <> s.0 <> s.0
    8 -> s.1 <> s.0 <> s.0 <> s.0
    9 -> s.0 <> s.2
    _ -> ""
  }
}

pub fn convert(number n: Int) -> String {
  let thousands = n / 1000
  let n = n % 1000
  let hundred = n / 100
  let n = n % 100
  let tens = n / 10
  let n = n % 10

  symbolize(thousands, #("M", "%", "%"))
  <> symbolize(hundred, #("C", "D", "M"))
  <> symbolize(tens, #("X", "L", "C"))
  <> symbolize(n, #("I", "V", "X"))
}
