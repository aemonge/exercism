fn rule(a: Float, b: Float, c: Float) -> Bool {
  { a +. b } >=. c
}

fn is_valid_triangle(a: Float, b: Float, c: Float) -> Bool {
  case a, b, c {
    0.0, _, _ -> False
    _, 0.0, _ -> False
    _, _, 0.0 -> False
    _, _, _ -> rule(a, b, c) && rule(b, c, a) && rule(c, a, b)
  }
}

/// An equilateral triangle has all three sides the same length.
pub fn equilateral(a: Float, b: Float, c: Float) -> Bool {
  case a {
    0.0 -> False
    _ -> is_valid_triangle(a, b, c) && a == b && b == c && a == c
  }
}

/// An isosceles triangle has at least two sides the same length.
/// (It is sometimes specified as having exactly two sides the same length, but for the
/// purposes of this exercise we'll say at least two.)
pub fn isosceles(a: Float, b: Float, c: Float) -> Bool {
  let a_matches = a == b || a == c
  let b_matches = b == a || b == c
  let c_matches = c == b || a == c

  is_valid_triangle(a, b, c)
  && {
    a_matches && b_matches || b_matches && c_matches || c_matches && a_matches
  }
}

/// A scalene triangle has all sides of different lengths.
pub fn scalene(a: Float, b: Float, c: Float) -> Bool {
  is_valid_triangle(a, b, c) && a != b && b != c && a != c
}
