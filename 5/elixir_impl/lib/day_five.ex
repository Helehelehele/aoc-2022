defmodule DayFive do
  def parse_containers(input) do
    input
    |> String.codepoints()
    |> Enum.chunk_every(4)
    |> Enum.map(fn line ->
      line |> Enum.map(&String.replace(&1, ["[", "]", " "], "")) |> Enum.join()
    end)
  end

  def parse_moves(input) do
    input
    |> String.split()
    |> Enum.flat_map(fn line ->
      line
      |> (fn string ->
            case Integer.parse(string) do
              {int, _rest} -> [int]
              :error -> []
            end
          end).()
    end)
  end

  def parse_input do
    File.read!("input.txt")
    |> String.split("\r\n")
    |> Enum.split_while(fn x -> x != "" end)
    |> (fn {container_lines, move_lines} ->
          {
            container_lines |> Enum.drop(-1),
            move_lines |> Enum.drop(1)
          }
        end).()
    |> (fn {container_lines, move_lines} ->
          {
            container_lines
            |> Enum.map(&parse_containers/1)
            |> List.zip()
            |> Enum.map(&Tuple.to_list/1)
            |> Enum.map(&Enum.reverse/1)
            |> Enum.map(&Enum.filter(&1, fn x -> x != "" end)),
            move_lines |> Enum.map(&parse_moves/1)
          }
        end).()
  end

  def part_one do
    parse_input()
    |> (fn {container_cols, moves} ->
          moves
          |> Enum.reduce(container_cols, fn move, acc ->
            acc
            |> Enum.with_index(1)
            |> Enum.map_reduce([], fn {col, idx}, acc ->
              # Move from source to accumulator
              case move do
                [amount, ^idx, _] -> col |> Enum.split(-amount)
                _ -> {col, acc}
              end
            end)
            |> (fn {containers, moving} ->
                  containers
                  |> Enum.with_index(1)
                  |> Enum.map(fn {col, idx} ->
                    # Move from accumulator to destination
                    case move do
                      [_, _, ^idx] -> col ++ (moving |> Enum.reverse())
                      _ -> col
                    end
                  end)
                end).()
          end)
        end).()
    |> Enum.map(&List.last/1)
    |> Enum.join()
    |> IO.inspect()
  end

  def part_two do
    parse_input()
    |> (fn {container_cols, moves} ->
          moves
          |> Enum.reduce(container_cols, fn move, acc ->
            acc
            |> Enum.with_index(1)
            |> Enum.map_reduce([], fn {col, idx}, acc ->
              case move do
                # Move from source to accumulator
                [amount, ^idx, _] -> col |> Enum.split(-amount)
                _ -> {col, acc}
              end
            end)
            |> (fn {containers, moving} ->
                  containers
                  |> Enum.with_index(1)
                  |> Enum.map(fn {col, idx} ->
                    # Move from accumulator to destination
                    case move do
                      [_, _, ^idx] -> col ++ moving
                      _ -> col
                    end
                  end)
                end).()
          end)
        end).()
    |> Enum.map(&List.last/1)
    |> Enum.join()
    |> IO.inspect()
  end
end
