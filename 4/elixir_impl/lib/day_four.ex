defmodule DayFour do
  def parse_range(range) do
    range
    |> String.split("-")
    |> Enum.map(&String.to_integer/1)
    |> Enum.reduce(&Range.new(&1, &2))
    |> Enum.reduce(MapSet.new(), &MapSet.put(&2, &1))
  end

  def part_one do
    File.read!("input.txt")
    |> String.split("\r\n")
    |> Enum.filter(&(&1 != ""))
    |> Enum.map(fn line ->
      line
      |> String.split(",")
      |> Enum.map(&parse_range/1)
    end)
    |> Enum.filter(fn [range1, range2] ->
      intersection = MapSet.intersection(range1, range2)

      intersection_size = MapSet.size(intersection)
      range1_size = MapSet.size(range1)
      range2_size = MapSet.size(range2)

      intersection_size == range1_size || intersection_size == range2_size
    end)
    |> Enum.count()
    |> IO.inspect()
  end

  def part_two do
    File.read!("input.txt")
    |> String.split("\r\n")
    |> Enum.filter(&(&1 != ""))
    |> Enum.map(fn line ->
      line
      |> String.split(",")
      |> Enum.map(&parse_range/1)
    end)
    |> Enum.filter(
      &(MapSet.intersection(Enum.at(&1, 0), Enum.at(&1, 1))
        |> MapSet.size()
        |> Kernel.>(0))
    )
    |> Enum.count()
    |> IO.inspect()
  end
end
