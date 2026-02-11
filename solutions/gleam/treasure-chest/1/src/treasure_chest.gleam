// Please define the TreasureChest generic custom type
pub type TreasureChest(s, x) {
  Password(String)
  TreasureChest(x)
}

// Please define the UnlockResult generic custom type
pub type UnlockResult(x) {
  Unlocked(x)
  WrongPassword
}

pub fn get_treasure(
  chest: TreasureChest(p, treasure),
  password: String,
) -> UnlockResult(treasure) {
  case chest {
    Chest(p, treasure) if p == password -> Unlocked(treasure)
    _ -> WrongPassword
  }
}
