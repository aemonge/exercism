defmodule WineCellar do
  # @wines [
  #   white: {"Chardonnay", 2015, "Italy"},
  #   white: {"Pinot grigio", 2017, "Germany"},
  #   red: {"Pinot noir", 2016, "France"},
  #   rose: {"Dornfelder", 2018, "Germany"}
  # ]

  def explain_colors do
    [
      {:white, "Fermented without skin contact."},
      red: "Fermented with skin contact using dark-colored grapes.",
      rose: "Fermented with some skin contact, but not enough to qualify as a red wine."
    ]
  end

  def filter_r(cellar, color, opts \\ []) do
    case opts do
      [{:year, year} | rest] ->
        Keyword.filter(cellar, fn {_, v} -> elem(v, 1) == year end)
        |> filter_r(color, rest)

      [{:country, country} | rest] ->
        Keyword.filter(cellar, fn {_, v} -> elem(v, 2) == country end)
        |> filter_r(color, rest)

      # [_ | rest] ->
      #   filter(cellar, color, rest)

      [] ->
        cellar
    end
    |> Keyword.filter(&(elem(&1, 0) == color))
  end

  def filter(cellar, color, opts \\ []),
    do:
      filter_r(cellar, color, opts)
      |> Keyword.values()

  # Keyword.filter(cellar, fn {k, _} -> k == color end)
  # Keyword.filter(cellar, &(elem(&1, 0) == color))
  # end

  # The functions below do not need to be modified.

  # defp filter_by_year(wines, year)
  # defp filter_by_year([], _year), do: []

  # defp filter_by_year([{_, year, _} = wine | tail], year) do
  #   [wine | filter_by_year(tail, year)]
  # end

  # defp filter_by_year([{_, _, _} | tail], year) do
  #   filter_by_year(tail, year)
  # end

  # defp filter_by_country(wines, country)
  # defp filter_by_country([], _country), do: []

  # defp filter_by_country([{_, _, country} = wine | tail], country) do
  #   [wine | filter_by_country(tail, country)]
  # end

  # defp filter_by_country([{_, _, _} | tail], country) do
  #   filter_by_country(tail, country)
  # end
end
