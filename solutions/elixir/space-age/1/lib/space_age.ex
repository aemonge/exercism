defmodule SpaceAge do
  @moduledoc """
  Planet                           │Orbital period in Earth Years
  ─────────────────────────────────┼──────────────────────────────────────────────────────
  Mercury                          │0.2408467
  Venus                            │0.61519726
  Earth                            │1.0
  Mars                             │1.8808158
  Jupiter                          │11.862615
  Saturn                           │29.447498
  Uranus                           │84.016846
  Neptune                          │164.79132
  """
  @type planet ::
          :mercury
          | :venus
          | :earth
          | :mars
          | :jupiter
          | :saturn
          | :uranus
          | :neptune

  defguardp is_planet(type)
            when type in [
                   :mercury,
                   :venus,
                   :earth,
                   :mars,
                   :jupiter,
                   :saturn,
                   :uranus,
                   :neptune
                 ]

  @doc """
  Return the number of years a person that has lived for 'seconds' seconds is
  aged on 'planet', or an error if 'planet' is not a planet.
  """
  @spec age_on(planet, pos_integer) :: {:ok, float} | {:error, String.t()}
  def age_on(planet, seconds) when is_planet(planet) do
    years = seconds / 31_557_600

    cond do
      planet == :mercury -> {:ok, years / 0.2408467}
      planet == :venus -> {:ok, years / 0.61519726}
      planet == :earth -> {:ok, years}
      planet == :mars -> {:ok, years / 1.8808158}
      planet == :jupiter -> {:ok, years / 11.862615}
      planet == :saturn -> {:ok, years / 29.447498}
      planet == :uranus -> {:ok, years / 84.016846}
      planet == :neptune -> {:ok, years / 164.79132}
    end
  end

  def age_on(_, _) do
    {:error, "not a planet"}
  end
end
