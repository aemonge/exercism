import gleam/list

fn factors_rec(value: Int, idx: Int, acc: List(Int)) -> List(Int) {
  let rest = value % idx
  let end = value == idx

  case rest, end {
    _, True -> list.append(acc, [idx])
    0, False -> {
      factors_rec({ value / idx }, 2, list.append(acc, [idx]))
    }
    _, False -> factors_rec(value, { idx + 1 }, acc)
  }
}

pub fn factors(value: Int) -> List(Int) {
  case value {
    1 -> []
    _ -> factors_rec(value, 2, [])
  }
}
