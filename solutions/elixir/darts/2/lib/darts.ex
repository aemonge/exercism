defmodule Darts do
  @type position :: {number, number}

  @spec _score(distance :: float) :: integer
  defp _score(radius) when radius <= 1.0, do: 10
  defp _score(radius) when radius <= 5.0, do: 5
  defp _score(radius) when radius <= 10.0, do: 1
  defp _score(_), do: 0

  @spec radius(position) :: float
  defp radius({x, y}) do
    (x ** 2 + y ** 2)
    |> Kernel.*(1.0)
    |> Float.pow(1 / 2)
  end

  @doc """
  Calculate the score of a single dart hitting a target
  """
  @spec score(position) :: integer
  def score(x), do: x |> radius() |> _score()
end
