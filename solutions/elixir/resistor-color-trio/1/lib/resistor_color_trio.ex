defmodule ResistorColorTrio do
  @spec code(atom) :: integer()
  defp code(:black), do: 0
  defp code(:brown), do: 1
  defp code(:red), do: 2
  defp code(:orange), do: 3
  defp code(:yellow), do: 4
  defp code(:green), do: 5
  defp code(:blue), do: 6
  defp code(:violet), do: 7
  defp code(:grey), do: 8
  defp code(:white), do: 9

  @doc """
  Calculate a resistance value from two colors
  """
  @spec value(colors :: [atom]) :: integer
  def value([a, b, c | _]),
    do:
      ((value([a, b]) |> Integer.to_string()) <> String.duplicate("0", code(c)))
      |> Integer.parse()
      |> elem(0)

  def value(colors), do: Integer.parse(value(colors, "") |> String.reverse()) |> elem(0)

  @spec value(colors :: [atom], acc :: String.t()) :: String.t()
  def value([], acc), do: acc
  def value([color | rest], acc), do: value(rest, Integer.to_string(code(color)) <> acc)

  @doc """
  Calculate the resistance value in ohms from resistor colors
  """
  @spec label(colors :: [atom]) :: {number, :ohms | :kiloohms | :megaohms | :gigaohms}
  def label(colors) do
    value = value(colors)

    cond do
      value >= 1_000_000_000 -> {value / 1_000_000_000, :gigaohms}
      value >= 1_000_000 -> {value / 1_000_000, :megaohms}
      value >= 1000 -> {value / 1000, :kiloohms}
      true -> {value, :ohms}
    end
  end
end
