import gleam/float
import gleam/int

pub fn pence_to_pounds(pence: Int) -> Float {
  let pence_f = int.to_float(pence)
  pence_f /. 100.0
}

pub fn pounds_to_string(pounds: Float) -> String {
  "£" <> float.to_string(pounds)
}
