defmodule DayEleven do
  def parse_initial_items(input) do
    input
    |> String.split(", ")
    |> Enum.filter(&(&1 != ""))
    |> Enum.map(&String.to_integer/1)
  end

  def parse_operation(input) do
    input
    |> String.split("= ")
    |> List.last()
    |> String.split(" ")
    |> Enum.take(-2)
    |> List.to_tuple()
  end

  def parse_test(input) do
    input
    |> String.split(" ")
    |> List.last()
    |> String.to_integer()
  end

  def parse_if_clause(input) do
    input
    |> String.split(" ")
    |> List.last()
    |> String.to_integer()
  end

  def parse_monkeys(chunk) do
    chunk
    |> Enum.map(&String.trim/1)
    |> Enum.drop(1)
    |> Enum.filter(&(&1 != ""))
    |> Enum.map(&String.split(&1, ": "))
    |> Enum.reduce(%{}, fn [key, value], acc ->
      case key do
        "Starting items" ->
          acc |> Map.put(:items, parse_initial_items(value))

        "Operation" ->
          acc |> Map.put(:operation, parse_operation(value))

        "Test" ->
          acc |> Map.put(:test, parse_test(value))

        "If true" ->
          acc |> Map.put(:if_true, parse_if_clause(value))

        "If false" ->
          acc |> Map.put(:if_false, parse_if_clause(value))
      end
    end)
    |> Map.put(:inspection_count, 0)
  end

  def read_monkeys do
    File.read!("input.txt")
    |> String.trim()
    |> String.split("\r\n")
    |> Enum.chunk_every(7)
    |> Enum.map(&parse_monkeys/1)
  end

  def perform_operation(item, operation) do
    case operation do
      {"+", "old"} -> item + item
      {"*", "old"} -> item * item
      {"+", number} -> item + String.to_integer(number)
      {"*", number} -> item * String.to_integer(number)
    end
  end

  def relief(operation_result, relief_level, :relaxed) do
    operation_result |> div(relief_level) |> floor()
  end

  def relief(operation_result, divisor, :not_relaxed) do
    rem(operation_result, divisor)
  end

  def perform_test(worry_level, test) do
    rem(worry_level, test) == 0
  end

  def get_divisor(monkeys) do
    monkeys
    |> Enum.map(&Map.get(&1, :test))
    |> Enum.product()
  end

  def run_round(monkeys, divisor, is_relaxed) do
    monkeys
    |> Enum.with_index()
    |> Enum.reduce(monkeys, fn {monkey, monkey_index}, acc ->
      acc
      |> Enum.at(monkey_index)
      |> Map.get(:items)
      |> Enum.map(&perform_operation(&1, Map.get(monkey, :operation)))
      |> Enum.map(fn operation_result ->
        relief(operation_result, divisor, is_relaxed)
      end)
      |> Enum.map(fn relief_result ->
        if perform_test(relief_result, Map.get(monkey, :test)) do
          {relief_result, Map.get(monkey, :if_true)}
        else
          {relief_result, Map.get(monkey, :if_false)}
        end
      end)
      |> Enum.reduce(acc, fn {worry, new_monkey_idx}, new_acc ->
        new_acc
        |> List.update_at(new_monkey_idx, fn new_monkey ->
          new_monkey
          |> update_in([:items], &List.insert_at(&1, -1, worry))
        end)
        |> List.update_at(monkey_index, fn old_monkey ->
          old_monkey
          |> update_in([:inspection_count], &(&1 + 1))
          |> update_in([:items], fn _ -> [] end)
        end)
      end)
    end)
  end

  def part_one do
    1..20
    |> Enum.reduce(read_monkeys(), fn _, acc ->
      acc |> run_round(3, :relaxed)
    end)
    |> Enum.map(&Map.get(&1, :inspection_count))
    |> Enum.sort(&>=/2)
    |> Enum.take(2)
    |> Enum.product()
    |> IO.inspect()
  end

  def part_two do
    1..10000
    |> Enum.reduce(read_monkeys(), fn _, acc ->
      acc |> run_round(get_divisor(acc), :not_relaxed)
    end)
    |> Enum.map(&Map.get(&1, :inspection_count))
    |> Enum.sort(&>=/2)
    |> Enum.take(2)
    |> Enum.product()
    |> IO.inspect()
  end
end
