defmodule DayThirteen do
  def parse_packet(packet) do
    packet_chars = to_charlist(packet)

    all_digits = Enum.all?(packet_chars, fn ch -> ch in ?0..?9 end)
    ends_w_close_bracket = List.last(packet_chars) == ?]

    case packet_chars do
      _digits when all_digits ->
        String.to_integer(packet)

      [?[, ?]] ->
        []

      [?[ | rest] when ends_w_close_bracket ->
        {packets, packet, _} =
          Enum.reduce(rest |> Enum.drop(-1), {[], "", 0}, fn curr, acc ->
            {packets, packet, bracket_level} = acc

            case curr do
              ?, when acc == {packets, packet, 0} ->
                {packets ++ [parse_packet(packet)], "", 0}

              ?] ->
                bracket_level = bracket_level - 1
                packet = packet <> List.to_string([curr])

                {packets, packet, bracket_level}

              ?[ ->
                bracket_level = bracket_level + 1
                packet = packet <> List.to_string([curr])

                {packets, packet, bracket_level}

              _ ->
                packet = packet <> List.to_string([curr])

                {packets, packet, bracket_level}
            end
          end)

        packets ++ [parse_packet(packet)]

      _ ->
        raise "Invalid packet: #{packet}"
    end
  end

  def compare_packets(left, right) do
    case {left, right} do
      {l, r} when is_integer(l) and is_integer(r) ->
        if l == r do
          :equal
        else
          if l < r do
            :less_than
          else
            :greater_than
          end
        end

      {l, r} when is_list(l) and is_list(r) ->
        Enum.zip(l, r)
        |> Enum.map(fn {l, r} -> compare_packets(l, r) end)
        |> Enum.find(fn cmp -> cmp != :equal end)
        |> case do
          nil ->
            case {Enum.count(l), Enum.count(r)} do
              {l, r} when l < r -> :less_than
              {l, r} when l > r -> :greater_than
              _ -> :equal
            end

          cmp ->
            cmp
        end

      {l, r} when is_list(l) and is_integer(r) ->
        compare_packets(l, [r])

      {l, r} when is_integer(l) and is_list(r) ->
        compare_packets([l], r)

      _ ->
        raise "Invalid comparison: #{left} #{right}"
    end
  end

  def part_one do
    File.read!("input.txt")
    |> String.trim()
    |> String.split("\r\n\r\n")
    |> Enum.map(fn pair ->
      pair |> String.split("\r\n") |> Enum.filter(&(&1 != "")) |> Enum.map(&parse_packet/1)
    end)
    |> Enum.map(fn [left, right] -> compare_packets(left, right) end)
    |> Enum.with_index(1)
    |> Enum.filter(fn {cmp, _} -> cmp != :greater_than end)
    |> Enum.map(fn {_, idx} -> idx end)
    |> Enum.sum()
    |> IO.inspect()
  end

  def part_two do
    File.read!("input.txt")
    |> String.trim()
    |> String.split("\r\n\r\n")
    |> Enum.flat_map(fn pair ->
      pair |> String.split("\r\n") |> Enum.filter(&(&1 != "")) |> Enum.map(&parse_packet/1)
    end)
    |> Kernel.++([[[2]], [[6]]])
    |> Enum.sort(fn left, right -> compare_packets(left, right) == :less_than end)
    |> Enum.with_index(1)
    |> Enum.filter(fn {packet, _} -> packet == [[2]] or packet == [[6]] end)
    |> Enum.map(fn {_, idx} -> idx end)
    |> Enum.product()
    |> IO.inspect()
  end
end
