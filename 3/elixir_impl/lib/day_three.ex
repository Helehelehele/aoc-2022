defmodule DayThree do
  def get_priority(ch) do
    if ch in ?A..?Z do
      ch - ?A + 27
    else
      ch - ?a + 1
    end
  end

  def part_one do
    File.read!("input.txt")
    |> String.split("\r\n")
    |> Enum.filter(&(&1 != ""))
    |> Enum.map(fn rucksack ->
      rucksack
      |> String.split_at(String.length(rucksack) |> div(2))
      |> Tuple.to_list()
      |> Enum.map(fn compartment ->
        compartment
        |> to_charlist()
        |> MapSet.new()
      end)
      |> Enum.reduce(fn compartment, acc -> MapSet.intersection(compartment, acc) end)
      |> MapSet.to_list()
      |> Enum.reduce(0, fn ch, acc -> acc + get_priority(ch) end)
    end)
    |> Enum.sum()
  end

  def part_two do
    File.read!("input.txt")
    |> String.split("\r\n")
    |> Enum.filter(&(&1 != ""))
    |> Enum.chunk_every(3)
    |> Enum.map(fn chunk ->
      chunk
      |> Enum.map(&(to_charlist(&1) |> MapSet.new()))
      |> Enum.reduce(fn rucksack, acc -> MapSet.intersection(rucksack, acc) end)
      |> MapSet.to_list()
      |> Enum.reduce(0, fn ch, acc -> acc + get_priority(ch) end)
    end)
    |> Enum.sum()
  end
end
