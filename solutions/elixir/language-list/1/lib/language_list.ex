defmodule LanguageList do
  def new() do
    []
  end

  def add(list, language) do
    [language | list]
  end

  def remove([_ | rest]) do
    rest
  end

  def first([head | _]) do
    head
  end

  def count([]) do
    0
  end

  # purposely avoiding length(rest)
  def count([_ | rest]) do
    1 + count(rest)
  end

  def functional_list?(list) do
    "Elixir" in list
  end
end
