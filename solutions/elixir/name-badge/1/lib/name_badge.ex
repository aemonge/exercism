defmodule NameBadge do
  def print(id, name, department) do
    department = if is_nil(department), do: "owner", else: department

    if is_nil(id) do
      "#{name} - #{department |> String.upcase()}"
    else
      "[#{id}] - #{name} - #{department |> String.upcase()}"
    end
  end
end
