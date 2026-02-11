defmodule RPG.CharacterSheet do
  def welcome() do
    IO.puts("Welcome! Let's fill out your character sheet together.")
  end

  def ask_name() do
    IO.gets("What is your character's name?\n")
    |> String.trim()
  end

  def ask_class() do
    IO.gets("What is your character's class?\n")
    |> String.trim()
  end

  def ask_level() do
    IO.gets("What is your character's level?\n")
    |> String.trim()
    |> Integer.parse()
    |> elem(0)
  end

  @doc """
  # > Welcome! Let's fill out your character sheet together.
  # > What is your character's name?
  # < Mathilde
  # > What is your character's class?
  # < healer
  # > What is your character's level?
  # < 2
  # > Your character: %{class: "healer", level: 2, name: "Mathilde"}
  # => %{class: "healer", level: 2, name: "Mathilde"}
  """
  def run() do
    welcome()
    name = ask_name()
    class = ask_class()
    level = ask_level()
    # ERROR: character = %{class: ask_class(), level: ask_level(), name: ask_name()}
    character = %{class: class, level: level, name: name}
    IO.inspect(character, label: "Your character")
    character
  end
end
