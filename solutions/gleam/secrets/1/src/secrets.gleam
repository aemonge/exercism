pub fn secret_add(secret s: Int) -> fn(Int) -> Int {
  fn(x: Int) -> Int { x + s }
}

pub fn secret_subtract(secret s: Int) -> fn(Int) -> Int {
  fn(x: Int) -> Int { x - s }
}

pub fn secret_multiply(secret s: Int) -> fn(Int) -> Int {
  fn(x: Int) -> Int { x * s }
}

pub fn secret_divide(secret s: Int) -> fn(Int) -> Int {
  fn(x: Int) -> Int { x / s }
}

pub fn secret_combine(
  secret_function1 a: fn(Int) -> Int,
  secret_function2 b: fn(Int) -> Int,
) -> fn(Int) -> Int {
  fn(x: Int) -> Int { a(x) |> b() }
}
