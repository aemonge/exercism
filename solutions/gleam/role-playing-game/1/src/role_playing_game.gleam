import gleam/int
import gleam/option.{type Option}

pub type Player {
  Player(name: Option(String), level: Int, health: Int, mana: Option(Int))
}

pub fn introduce(player: Player) -> String {
  case player.name {
    option.None -> "Mighty Magician"
    option.Some(x) -> x
  }
}

// 7 Failed
pub fn revive(player: Player) -> Option(Player) {
  case player {
    Player(health: 0, level: l, ..) if l >= 10 ->
      option.Some(Player(
        health: 100,
        mana: option.Some(100),
        name: player.name,
        level: player.level,
      ))
    Player(health: 0, ..) ->
      option.Some(Player(
        health: 100,
        mana: player.mana,
        name: player.name,
        level: player.level,
      ))
    _ -> option.None
  }
}

pub fn cast_spell(player: Player, cost: Int) -> #(Player, Int) {
  case player {
    Player(mana: option.None, ..) -> {
      let health = int.max({ player.health - cost }, 0)
      #(
        Player(
          name: player.name,
          health:,
          mana: player.mana,
          level: player.level,
        ),
        0,
      )
    }
    Player(mana: option.Some(x), ..) if x < cost -> #(player, 0)
    Player(mana: option.Some(x), ..) if x >= cost -> {
      #(
        Player(
          name: player.name,
          health: player.health,
          mana: option.Some(x - cost),
          level: player.level,
        ),
        { cost * 2 },
      )
    }
    _ -> #(player, 0)
  }
}
