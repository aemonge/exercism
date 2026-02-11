defmodule FreelancerRates do
  def daily_rate(hourly_rate) do
    hourly_rate * 8.0
  end

  def apply_discount(before_discount, discount) do
    before_discount * (1 - discount / 100)
  end

  def monthly_rate(hourly_rate, discount) do
    brute = 22 * daily_rate(hourly_rate)
    ceil(apply_discount(brute, discount))
  end

  @doc """
  Implement a function that takes a budget, an hourly rate, and a discount, and calculates
  how many days of work that covers.
  ## Example
  >>> FreelancerRates.days_in_budget(20000, 80, 11.0)
  <<< 35.1
  """
  def days_in_budget(budget, hourly_rate, discount) do
    real_budget = budget * (1.0 + discount / 100)
    x = trunc(real_budget / 8.0)
    days = x / hourly_rate
    # expecting 1.25 -> 1.2
    Float.floor(days, 1)
  end
end
