defmodule EliudsEggs do
  @doc """
  Given the number, count the number of eggs.
  """
  @spec egg_count(number :: integer()) :: non_neg_integer()
  def egg_count(number) do
    Integer.digits(number, 2)
    |> Enum.reduce(0, fn x, acc -> x + acc end)
  end
end
