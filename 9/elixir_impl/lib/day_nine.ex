defmodule DayNine do
  def get_head_movements do
    File.read!("input.txt")
    |> String.split("\r\n")
    |> Enum.drop(-1)
    |> Enum.map(&(String.split(&1, " ") |> Enum.take(2)))
    |> Enum.map(fn [direction, distance] ->
      {direction, distance |> String.to_integer()}
    end)
    |> Enum.reduce([{0, 0}], fn {direction, distance}, acc ->
      case direction do
        "R" ->
          Enum.reduce(1..distance, acc, fn _, acc ->
            {x, y} = List.last(acc)
            acc ++ [{x + 1, y}]
          end)

        "L" ->
          Enum.reduce(1..distance, acc, fn _, acc ->
            {x, y} = List.last(acc)
            acc ++ [{x - 1, y}]
          end)

        "U" ->
          Enum.reduce(1..distance, acc, fn _, acc ->
            {x, y} = List.last(acc)
            acc ++ [{x, y + 1}]
          end)

        "D" ->
          Enum.reduce(1..distance, acc, fn _, acc ->
            {x, y} = List.last(acc)
            acc ++ [{x, y - 1}]
          end)
      end
    end)
  end

  def get_tail(head, current_tail) do
    difference = {
      (head |> elem(0)) - (current_tail |> elem(0)),
      (head |> elem(1)) - (current_tail |> elem(1))
    }

    {head_x, head_y} = head

    case difference do
      {x, y} when x in -1..1 and y in -1..1 -> current_tail
      {x, -2} when x in -1..1 -> {head_x, head_y + 1}
      {x, 2} when x in -1..1 -> {head_x, head_y - 1}
      {-2, y} when y in -1..1 -> {head_x + 1, head_y}
      {2, y} when y in -1..1 -> {head_x - 1, head_y}
      {-2, -2} -> {head_x + 1, head_y + 1}
      {-2, 2} -> {head_x + 1, head_y - 1}
      {2, -2} -> {head_x - 1, head_y + 1}
      {2, 2} -> {head_x - 1, head_y - 1}
      _ -> :error
    end
  end

  def part_one do
    get_head_movements()
    |> Enum.reduce([{0, 0}], fn head, acc ->
      acc ++ [get_tail(head, List.last(acc))]
    end)
    |> MapSet.new()
    |> MapSet.size()
    |> IO.inspect()
  end

  def part_two do
    get_head_movements()
    |> (fn head_moves ->
          visits = head_moves

          visits =
            for _ <- 1..9, reduce: visits do
              v ->
                v
                |> Enum.reduce([{0, 0}], fn head, acc ->
                  acc ++ [get_tail(head, List.last(acc))]
                end)
            end

          visits
        end).()
    |> MapSet.new()
    |> MapSet.size()
    |> IO.inspect()
  end
end
