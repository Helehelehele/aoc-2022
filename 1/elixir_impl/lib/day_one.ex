defmodule DayOne do
  def get_calories do
    top_calories =
      File.read!("input.txt")
      |> String.split("\r\n\r\n")
      |> Enum.with_index(1)
      |> Enum.map(fn {elf, index} ->
        elf
        |> String.split("\r\n")
        |> Enum.filter(fn line -> line != "" end)
        |> Enum.map(fn line -> String.to_integer(line) end)
        |> Enum.reduce(fn cur, acc -> cur + acc end)
        |> (fn calories -> {index, calories} end).()
      end)
      |> Enum.sort_by(fn {_, calories} -> calories end, &>=/2)
      |> Enum.take(3)

    top_calories
    |> Enum.at(0)
    |> (&"Elf #{elem(&1, 0)} has the most calories: #{elem(&1, 1)}").()
    |> IO.puts()

    top_calories
    |> Enum.reduce(0, fn {_, calories}, acc -> calories + acc end)
    |> (&"Total calories: #{&1}").()
    |> IO.puts()

    :ok
  end
end
