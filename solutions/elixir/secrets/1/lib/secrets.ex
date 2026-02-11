defmodule Secrets do
  def secret_add(secret) do
    fn x -> secret + x end
  end

  def secret_subtract(secret) do
    fn x -> x - secret end
  end

  def secret_multiply(secret) do
    &(secret * &1)
  end

  def secret_divide(secret) do
    &Kernel.div(&1, secret)
  end

  def secret_and(secret) do
    &Bitwise.band(secret, &1)
  end

  def secret_xor(secret) do
    &Bitwise.bxor(secret, &1)
  end

  def secret_combine(secret_function1, secret_function2) do
    fn x ->
      y = secret_function1.(x)
      secret_function2.(y)
    end

    # &secret_function2.(secret_function1.(&1))
  end
end
