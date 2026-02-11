import gleam/option.{type Option}

pub fn two_fer(name: Option(String)) -> String {
  case name {
    option.None -> "One for you, one for me."
    option.Some(x) -> "One for " <> x <> ", one for me."
  }
}
