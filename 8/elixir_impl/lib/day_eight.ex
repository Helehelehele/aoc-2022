defmodule DayEight do
  def part_one do
    File.read!("input.txt")
    |> String.split("\r\n", trim: true)
    |> Enum.with_index()
    |> Enum.reduce(%{}, fn {row, y}, acc ->
      row
      |> String.split("", trim: true)
      |> Enum.map(&String.to_integer(&1))
      |> Enum.with_index()
      |> Enum.reduce(acc, fn {cell, x}, a ->
        Map.put(a, {x, y}, cell)
      end)
    end)
    |> (fn trees ->
          trees
          |> Map.keys()
          |> Enum.filter(fn {x, y} ->
            check_visible(trees, :up, {x, y}) ||
              check_visible(trees, :down, {x, y}) ||
              check_visible(trees, :left, {x, y}) ||
              check_visible(trees, :right, {x, y})
          end)
        end).()
    |> Enum.count()
    |> IO.inspect()
  end

  def check_visible(_, :up, {_, 0}), do: true

  def check_visible(trees, :up, {x, y}) do
    cur = Map.get(trees, {x, y})

    Enum.all?(0..(y - 1), fn i ->
      Map.get(trees, {x, i}) < cur
    end)
  end

  def check_visible(trees, :down, {x, y}) do
    cur = Map.get(trees, {x, y})

    size = trunc(:math.sqrt(Enum.count(trees)))

    y == size - 1 ||
      Enum.all?((y + 1)..(size - 1), fn i ->
        Map.get(trees, {x, i}) < cur
      end)
  end

  def check_visible(_, :left, {0, _}), do: true

  def check_visible(trees, :left, {x, y}) do
    cur = Map.get(trees, {x, y})

    Enum.all?(0..(x - 1), fn i ->
      Map.get(trees, {i, y}) < cur
    end)
  end

  def check_visible(trees, :right, {x, y}) do
    cur = Map.get(trees, {x, y})

    size = trunc(:math.sqrt(Enum.count(trees)))

    x == size - 1 ||
      Enum.all?((x + 1)..(size - 1), fn i ->
        Map.get(trees, {i, y}) < cur
      end)
  end

  def part_two do
    File.read!("input.txt")
    |> String.split("\r\n", trim: true)
    |> Enum.with_index()
    |> Enum.reduce(%{}, fn {row, y}, acc ->
      row
      |> String.split("", trim: true)
      |> Enum.map(&String.to_integer(&1))
      |> Enum.with_index()
      |> Enum.reduce(acc, fn {cell, x}, a ->
        Map.put(a, {x, y}, cell)
      end)
    end)
    |> (fn trees ->
          trees
          |> Map.keys()
          |> Enum.reduce(0, fn {x, y}, acc ->
            max(
              acc,
              count_visible_trees(trees, :up, {x, y}) *
                count_visible_trees(trees, :down, {x, y}) *
                count_visible_trees(trees, :left, {x, y}) *
                count_visible_trees(trees, :right, {x, y})
            )
          end)
        end).()
    |> IO.inspect()
  end

  def count_visible_trees(_, :up, {_, 0}), do: 0

  def count_visible_trees(trees, :up, {x, y}) do
    cur = Map.get(trees, {x, y})

    Enum.reduce_while((y - 1)..0, 0, fn i, acc ->
      if Map.get(trees, {x, i}) < cur do
        {:cont, acc + 1}
      else
        {:halt, acc + 1}
      end
    end)
  end

  def count_visible_trees(trees, :down, {x, y}) do
    cur = Map.get(trees, {x, y})

    size = trunc(:math.sqrt(Enum.count(trees)))

    if y == size - 1 do
      0
    else
      Enum.reduce_while((y + 1)..(size - 1), 0, fn i, acc ->
        if Map.get(trees, {x, i}) < cur do
          {:cont, acc + 1}
        else
          {:halt, acc + 1}
        end
      end)
    end
  end

  def count_visible_trees(_, :left, {0, _}), do: 0

  def count_visible_trees(trees, :left, {x, y}) do
    cur = Map.get(trees, {x, y})

    Enum.reduce_while((x - 1)..0, 0, fn i, acc ->
      if Map.get(trees, {i, y}) < cur do
        {:cont, acc + 1}
      else
        {:halt, acc + 1}
      end
    end)
  end

  def count_visible_trees(trees, :right, {x, y}) do
    cur = Map.get(trees, {x, y})

    size = trunc(:math.sqrt(Enum.count(trees)))

    if x == size - 1 do
      0
    else
      Enum.reduce_while((x + 1)..(size - 1), 0, fn i, acc ->
        if Map.get(trees, {i, y}) < cur do
          {:cont, acc + 1}
        else
          {:halt, acc + 1}
        end
      end)
    end
  end
end
