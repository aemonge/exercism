import gleam/string

pub fn message(log_line: String) -> String {
  {
    case log_line {
      "[ERROR]: " <> message -> message
      "[WARNING]: " <> message -> message
      "[INFO]: " <> message -> message
      _ -> "Excpetion"
    }
  }
  |> string.trim
}

pub fn log_level(log_line: String) -> String {
  case log_line {
    "[ERROR]:" <> _ -> "error"
    "[WARNING]:" <> _ -> "warning"
    "[INFO]:" <> _ -> "info"
    _ -> "Excpetion"
  }
}

pub fn reformat(log_line: String) -> String {
  let msg = message(log_line)
  let level = log_level(log_line)

  msg <> " (" <> level <> ")"
}
