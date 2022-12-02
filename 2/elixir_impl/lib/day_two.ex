defmodule DayTwo do
  def get_rounds do
    File.read!("input.txt")
    |> String.split("\r\n")
    |> Enum.filter(&(&1 != ""))
    |> Enum.map(&String.split(&1, " "))
  end

  def get_round_result(round) do
    [opponent, me] = round

    case {opponent, me} do
      {"A", "X"} -> 3 + 1
      {"A", "Y"} -> 6 + 2
      {"A", "Z"} -> 0 + 3
      {"B", "X"} -> 0 + 1
      {"B", "Y"} -> 3 + 2
      {"B", "Z"} -> 6 + 3
      {"C", "X"} -> 6 + 1
      {"C", "Y"} -> 0 + 2
      {"C", "Z"} -> 3 + 3
    end
  end

  def get_actual_moves(round) do
    [opponent, result] = round

    case {opponent, result} do
      {"A", "X"} -> ["A", "Z"]
      {"A", "Y"} -> ["A", "X"]
      {"A", "Z"} -> ["A", "Y"]
      {"B", "X"} -> ["B", "X"]
      {"B", "Y"} -> ["B", "Y"]
      {"B", "Z"} -> ["B", "Z"]
      {"C", "X"} -> ["C", "Y"]
      {"C", "Y"} -> ["C", "Z"]
      {"C", "Z"} -> ["C", "X"]
    end
  end

  def part_one do
    get_rounds()
    |> Enum.map(&get_round_result/1)
    |> Enum.sum()
  end

  def part_two do
    get_rounds()
    |> Enum.map(&get_actual_moves/1)
    |> Enum.map(&get_round_result/1)
    |> Enum.sum()
  end
end
