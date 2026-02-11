import gleam/io

// Please define the TreasureChest generic custom type
pub type TreasureChest(x) {
  TreasureChest(pass: String, x)
}

// Please define the UnlockResult generic custom type
pub type UnlockResult(x) {
  Unlocked(x)
  WrongPassword
  GoodPassword
}

pub fn get_treasure(
  chest: TreasureChest(x),
  password: String,
) -> UnlockResult(treasure) {
  io.debug(chest.pass)
  case chest.pass {
    p if p == password -> GoodPassword
    _ -> WrongPassword
  }
}
