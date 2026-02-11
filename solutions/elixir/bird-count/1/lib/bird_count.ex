defmodule BirdCount do
  def today([]) do
    nil
  end

  def today([tcount | _]) do
    tcount
  end

  def increment_day_count([]) do
    [1]
  end

  def increment_day_count([today | rest]) do
    [today + 1 | rest]
  end

  def has_day_without_birds?([]) do
    false
  end

  def has_day_without_birds?([day | rest]) do
    day == 0 or has_day_without_birds?(rest)
  end

  defp total([], acc), do: acc
  defp total([count | rest], acc), do: total(rest, acc + count)

  def total(list), do: total(list, 0)

  defp busy_days([], acc), do: acc

  defp busy_days([count | rest], acc) do
    # busy_days(rest, acc + (count >= 5 && 1) || 0)

    if count >= 5 do
      busy_days(rest, acc + 1)
    else
      busy_days(rest, acc)
    end
  end

  def busy_days(list), do: busy_days(list, 0)
end
