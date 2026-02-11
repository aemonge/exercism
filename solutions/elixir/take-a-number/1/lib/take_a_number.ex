defmodule TakeANumber do
  @spec loop(state :: Integer) :: nil
  defp loop(state) do
    receive do
      {:report_state, sender_pid} ->
        send(sender_pid, state)
        loop(state)

      {:take_a_number, sender_pid} ->
        state = state + 1
        send(sender_pid, state)
        loop(state)

      {:stop, _} ->
        nil

      _ ->
        loop(state)
    end
  end

  def start() do
    spawn(fn -> loop(0) end)
  end
end
