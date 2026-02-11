defmodule LogLevel do
  @doc """
  Log code                        │Log label          │Supported in legacy apps?
  ────────────────────────────────┼───────────────────┼───────────────────────────────────
  0                               │trace              │no
  1                               │debug              │yes
  2                               │info               │yes
  3                               │warning            │yes
  4                               │error              │yes
  5                               │fatal              │no
  other / not supported           │unknown            │-
  """
  def to_label(level, legacy?) do
    cond do
      not legacy? and level == 0 -> :trace
      level == 1 -> :debug
      level == 2 -> :info
      level == 3 -> :warning
      level == 4 -> :error
      not legacy? and level == 5 -> :fatal
      true -> :unknown
    end
  end

  @doc """
  Implement the LogLevel.alert_recipient/2 function to determine to whom the alert needs
  to be sent. The function should take an integer code and a boolean flag telling you if
  the log comes from a legacy app, and return the name of the recipient as an atom.

  Use the LogLevel.to_label/2 function from the previous task. If the log label is error
  or fatal, send the alert to the ops team. If you receive a log with an unknown label
  from a legacy system, send the alert to the dev1 team, other unknown labels should be
  sent to the dev2 team. All other log labels can be safely ignored by returning false.
  """
  def alert_recipient(level, legacy?) do
    label = to_label(level, legacy?)

    cond do
      label == :error or label == :fatal ->
        :ops

      label == :unknown ->
        cond do
          legacy? -> :dev1
          true -> :dev2
        end
      true ->
        false
    end
  end
end
