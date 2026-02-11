defmodule CollatzConjecture do
  @doc """
  calc/1 takes an integer and returns the number of steps required to get the
  number to 1 when following the rules:
    - if number is odd, multiply with 3 and add 1
    - if number is even, divide by 2
  """
  def calc(1, steps_acc), do: steps_acc
  def calc(0, steps_acc), do: steps_acc

  def calc(input, steps_acc) when rem(input, 2) == 0 do
    calc(div(input, 2), steps_acc + 1)
  end

  def calc(input, steps_acc) do
    calc(input * 3 + 1, steps_acc + 1)
  end

  @spec calc(input :: pos_integer()) :: non_neg_integer()
  def calc(input) when is_integer(input) and input > 0, do: calc(input, 0)
end
