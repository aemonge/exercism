defmodule RationalNumbers do
  @type rational :: {integer, integer}

  defp is_prime?(number, ix)
  defp is_prime?(1, _), do: true
  defp is_prime?(2, _), do: true
  defp is_prime?(3, _), do: true
  defp is_prime?(_, 1), do: true

  defp is_prime?(number, ix) do
    if Integer.mod(number, ix) == 0 do
      false
    else
      is_prime?(number, ix - 1)
    end
  end

  defp is_prime?(number), do: is_prime?(number, number - 1)

  defp clean_factors([], acc), do: acc

  defp clean_factors([check | rest], acc) do
    if is_prime?(check) do
      clean_factors(rest, [check | acc])
    else
      clean_factors(rest, acc)
    end
  end

  @doc """
  Quick factors, using the inefficient way to get the prime factors
  of a number, by not removing already checked factors
  """
  # def factors(_, 2, factors), do: [2 | factors]
  def factors(divisor, number, factors) when divisor == number, do: [divisor | factors]

  def factors(divisor, number, factors) do
    if Integer.mod(number, divisor) == 0 do
      factors(divisor, Integer.floor_div(number, divisor), [divisor | factors])
    else
      factors(divisor + 1, number, factors)
    end
  end

  def factors(number) do
    factors(2, number, [])
    |> clean_factors([])
  end

  defp xor(a, b) do
    (a -- b) ++ (b -- a)
  end

  @doc """
  Reduce a rational to it's smallest factors
  """
  @spec reduce(a :: rational) :: rational
  def reduce({0, _}), do: {0, 1}
  def reduce({_, 0}), do: {0, 1}
  def reduce({n, d}) when n == d, do: {1, 1}

  def reduce({numerator, denominator}) do
    {n_factors, d_factors} = {factors(numerator), factors(denominator)}
    xor(n_factors)
    {n_factors -- d}
  end

  @doc """
  Absolute value of a rational number
  """
  @spec abs(a :: rational) :: rational
  def abs(a) do
    {numerator, denominator} = a

    if numerator < 0 do
      numerator = numerator * -1
    end

    if denominator < 0 do
      denominator = denominator * -1
    end

    {numerator, denominator}
    |> reduce
  end

  @doc """
  Add two rational numbers

  The sum of two rational numbers r₁ = a₁/b₁ and r₂ = a₂/b₂ is r₁ + r₂ = a₁/b₁ + a₂/b₂ =
  (a₁ * b₂ + a₂ * b₁) / (b₁ * b₂).
  """
  @spec add(a :: rational, b :: rational) :: rational
  def add({_, 0}, _), do: {0 / 1}
  def add(_, {_, 0}), do: {0 / 1}

  def add(a, b) do
    {numerator_a, denominator_a} = a
    {numerator_b, denominator_b} = b

    {denominator_b * numerator_a + denominator_a * numerator_b, denominator_a * denominator_b}
    |> reduce
  end

  @doc """
  Subtract two rational numbers
  The difference of two rational numbers r₁ = a₁/b₁ and r₂ = a₂/b₂ is r₁ - r₂ = a₁/b₁ -
  a₂/b₂ = (a₁ * b₂ - a₂ * b₁) / (b₁ * b₂).
  """
  @spec subtract(a :: rational, b :: rational) :: rational
  def subtract(a, b) do
  end

  @doc """
  Multiply two rational numbers
  The product (multiplication) of two rational numbers r₁ = a₁/b₁ and r₂ = a₂/b₂ is r₁ *
  r₂ = (a₁ * a₂) / (b₁ * b₂).
  """
  @spec multiply(a :: rational, b :: rational) :: rational
  def multiply(a, b) do
  end

  @doc """
  Divide two rational numbers
  Dividing a rational number r₁ = a₁/b₁ by another r₂ = a₂/b₂ is r₁ / r₂ = (a₁ * b₂) / (a₂
  * b₁) if a₂ is not zero.
  """
  @spec divide_by(num :: rational, den :: rational) :: rational
  def divide_by(num, den) do
  end

  @doc """
  Exponentiation of a rational number by an integer
  Exponentiation of a rational number r = a/b to a non-negative integer power n is r^n =
  (a^n)/(b^n).
  """
  @spec pow_rational(a :: rational, n :: integer) :: rational
  def pow_rational(a, n) do
  end

  @doc """
  Exponentiation of a real number by a rational number
  Exponentiation of a rational number r = a/b to a real (floating-point) number x is the
  quotient (a^x)/(b^x), which is a real number.

  Exponentiation of a real number x to a rational number r = a/b is x^(a/b) = root(x^a,
  b), where root(p, q) is the qth root of p.
  """
  @spec pow_real(x :: integer, n :: rational) :: float
  def pow_real(x, n) do
  end
end
