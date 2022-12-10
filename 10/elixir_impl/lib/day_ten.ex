defmodule DayTen do
  def part_one do
    File.read!("input.txt")
    |> String.trim()
    |> String.split("\r\n")
    |> Enum.reduce([], fn line, acc ->
      line
      |> String.split(" ", parts: 2)
      |> List.to_tuple()
      |> case do
        {"noop"} -> acc ++ [0]
        {"addx", val} -> acc ++ [0, String.to_integer(val)]
      end
    end)
    |> Enum.reduce([], fn val, acc ->
      case acc do
        [] -> [val + 1]
        [head | tail] -> [head + val | [head | tail]]
      end
    end)
    |> Enum.reverse()
    |> Enum.with_index(1)
    |> Enum.filter(fn {_, idx} -> (idx + 1) in [20, 60, 100, 140, 180, 220] end)
    |> Enum.map(fn {val, idx} -> (idx + 1) * val end)
    |> Enum.sum()
    |> IO.inspect()
  end

  def part_two do
    File.read!("input.txt")
    |> String.trim()
    |> String.split("\r\n")
    |> Enum.reduce([], fn line, acc ->
      line
      |> String.split(" ", parts: 2)
      |> List.to_tuple()
      |> case do
        {"noop"} -> acc ++ [0]
        {"addx", val} -> acc ++ [0, String.to_integer(val)]
      end
    end)
    |> Enum.with_index()
    |> Enum.reduce({List.duplicate(List.duplicate(" ", 40), 6), 1}, fn {val, idx}, acc ->
      dx = rem(idx, 40)
      dy = div(idx, 40) |> trunc()
      {grid, x} = acc

      case dx do
        should_paint when (should_paint - x) in -1..1 ->
          {
            List.replace_at(grid, dy, List.replace_at(Enum.at(grid, dy), dx, "X")),
            x + val
          }

        _ ->
          {
            List.replace_at(grid, dy, List.replace_at(Enum.at(grid, dy), dx, " ")),
            x + val
          }
      end
    end)
    |> elem(0)
    |> Enum.map(fn row -> Enum.join(row) end)
    |> Enum.each(fn row -> IO.puts(row) end)
  end
end
