// Please define the TreasureChest generic custom type
pub type TreasureChest(x) {
  TreasureChest(pass: String, treasure: x)
}

// Please define the UnlockResult generic custom type
pub type UnlockResult(x) {
  Unlocked(x)
  WrongPassword
  GoodPassword
}

pub fn get_treasure(
  chest: TreasureChest(treasure),
  password: String,
) -> UnlockResult(treasure) {
  case chest.pass {
    p if p == password -> Unlocked(chest.treasure)
    _ -> WrongPassword
  }
}
