defmodule KitchenCalculator do
  def get_volume({_, metric}) do
    metric
  end

  def to_milliliter({:cup, value}) do
    {:milliliter, value * 240}
  end

  def to_milliliter({:milliliter, value}) do
    {:milliliter, value}
  end

  def to_milliliter({:fluid_ounce, value}) do
    {:milliliter, value * 30}
  end

  def to_milliliter({:teaspoon, value}) do
    {:milliliter, value * 5}
  end

  def to_milliliter({:tablespoon, value}) do
    {:milliliter, value * 15}
  end

  def from_milliliter({:milliliter, volume_pair}, :milliliter) do
    {:milliliter, volume_pair}
  end

  def from_milliliter({:milliliter, volume_pair}, :cup) do
    {:cup, volume_pair / 240}
  end

  def from_milliliter({:milliliter, volume_pair}, :fluid_ounce) do
    {:fluid_ounce, volume_pair / 30}
  end

  def from_milliliter({:milliliter, volume_pair}, :teaspoon) do
    {:teaspoon, volume_pair / 5}
  end

  def from_milliliter({:milliliter, volume_pair}, :tablespoon) do
    {:tablespoon, volume_pair / 15}
  end

  def convert({:milliliter, from}, to_type) do
    from_milliliter({:milliliter, from}, to_type)
  end

  def convert({not_a_milliliter, value}, to_type) do
    milliliter = to_milliliter({not_a_milliliter, value})
    convert(milliliter, to_type)
  end
end
