defmodule Triangle do
  @type kind :: :equilateral | :isosceles | :scalene

  defguardp is_valid_triangle(a, b, c)
            when a + b >= c and b + c >= a and
                   a + c >= b

  defguardp are_positives(a, b, c) when a > 0 and b > 0 and c > 0

  defp count_equal_size({a, b, c}) do
    acc = if a == b, do: 1, else: 0
    acc = if a == c, do: acc + 1, else: acc
    acc = if b == c, do: acc + 1, else: acc
    acc
  end

  @doc """
  Return the kind of triangle of a triangle with 'a', 'b' and 'c' as lengths.
  """
  @spec kind(number, number, number) :: {:ok, kind} | {:error, String.t()}
  def kind(a, b, c) when is_valid_triangle(a, b, c) and are_positives(a, b, c) do
    case count_equal_size({a, b, c}) do
      3 -> :equilateral
      2 -> :isosceles
      1 -> :isosceles
      _ -> :scalene
    end
    |> then(&{:ok, &1})
  end

  def kind(a, b, c) when not are_positives(a, b, c),
    do: {:error, "all side lengths must be positive"}

  def kind(_, _, _), do: {:error, "side lengths violate triangle inequality"}
end
