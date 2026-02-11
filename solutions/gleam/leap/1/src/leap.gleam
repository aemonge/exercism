pub fn is_leap_year(year: Int) -> Bool {
  let rest = year % 4
  case rest {
    0 -> {
      let rest = year % 100
      case rest {
        0 -> {
          case { year % 400 } {
            0 -> True
            _ -> False
          }
        }
        _ -> True
      }
    }
    _ -> False
  }
}
