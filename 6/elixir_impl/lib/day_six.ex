defmodule DaySix do
  def part_one do
    File.read!("input.txt")
    |> String.codepoints()
    |> Enum.with_index()
    # Window the list with a sliding window of 4
    |> Enum.chunk_every(4, 1, :discard)
    |> Enum.find(fn window ->
      window
      |> Enum.map(fn {char, _} -> char end)
      |> MapSet.new()
      |> MapSet.size() == 4
    end)
    |> List.last()
    |> elem(1)
    |> Kernel.+(1)
    |> IO.inspect()
  end

  def part_two do
    File.read!("input.txt")
    |> String.codepoints()
    |> Enum.with_index()
    # Window the list with a sliding window of 4
    |> Enum.chunk_every(14, 1, :discard)
    |> Enum.find(fn window ->
      window
      |> Enum.map(fn {char, _} -> char end)
      |> MapSet.new()
      |> MapSet.size() == 14
    end)
    |> List.last()
    |> elem(1)
    |> Kernel.+(1)
    |> IO.inspect()
  end
end
