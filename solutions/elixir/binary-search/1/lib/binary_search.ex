defmodule BinarySearch do
  @spec split(list) :: {list, integer, list, integer}
  def split([x | []]), do: {[], x, [], 0}
  def split([x, y | []]), do: {[x], y, [], 1}
  def split([x, y, z | []]), do: {[x], y, [z], 1}

  def split(list_ge_four) do
    half_idx = (Enum.count(list_ge_four) / 2) |> trunc
    {left, [x | right]} = Enum.split(list_ge_four, half_idx)
    {left, x, right, half_idx}
  end

  @spec search(list, integer, integer) :: {:ok, integer} | :not_found
  defp search([], _, _), do: :not_found
  defp search([x | []], needle, acc) when x == needle, do: {:ok, acc}

  defp search(haystack, needle, acc) do
    {left, x, right, discarted_elements} = split(haystack)

    cond do
      needle < x -> search(left, needle, acc)
      needle > x -> search(right, needle, acc + discarted_elements + 1)
    end
  end

  @doc """
    Searches for a key in the tuple using the binary search algorithm.
    It returns :not_found if the key is not in the tuple.
    Otherwise returns {:ok, index}.

    ## Examples

      iex> BinarySearch.search({}, 2)
      :not_found

      iex> BinarySearch.search({1, 3, 5}, 2)
      :not_found

      iex> BinarySearch.search({1, 3, 5}, 5)
      {:ok, 2}

  """
  @spec search(tuple, integer) :: {:ok, integer} | :not_found
  def search({}, _), do: :not_found

  def search(haystack, needle) do
    search(Tuple.to_list(haystack), needle, 0)
  end
end
