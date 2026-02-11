defmodule Username do
  def sanitize([], acc), do: acc |> Enum.reverse()

  def sanitize([u | rest], acc) do
    case u do
      ?ä ->
        sanitize(rest, [?e, ?a | acc])

      ?ö ->
        sanitize(rest, [?e, ?o | acc])

      ?ü ->
        sanitize(rest, [?e, ?u | acc])

      ?ß ->
        sanitize(rest, [?s, ?s | acc])

      ?_ ->
        sanitize(rest, [u | acc])

      u when u in ?a..?z ->
        sanitize(rest, [u | acc])

      _ ->
        sanitize(rest, acc)

        # u when u in ?0..?9 -> sanitize(rest, acc)
        # u when u in ~c"*!$% " -> sanitize(rest, acc)
        # # u when u in [?*, ?!, ?$, ?%, ?\s] -> sanitize(rest, acc)
        # _ -> sanitize(rest, [u | acc])
    end
  end
  def sanitize(username), do: sanitize(username, [])
end
