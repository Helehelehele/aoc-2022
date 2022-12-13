defmodule DayTwelve do
  def parse_line(line) do
    line
    |> String.to_charlist()
    |> Enum.map(fn square ->
      case square do
        lowercase when lowercase in ?a..?z ->
          %{
            value: lowercase - ?a,
            is_endpoint: false,
            is_startpoint: false
          }

        ?E ->
          %{
            value: ?z - ?a,
            is_endpoint: true,
            is_startpoint: false
          }

        ?S ->
          %{
            value: ?a - ?a,
            is_endpoint: false,
            is_startpoint: true
          }

        _ ->
          IO.puts("Unknown square: #{square}")
          :error
      end
    end)
  end

  def get_neighbors({x, y}, {width, height}) do
    [
      {x - 1, y},
      {x + 1, y},
      {x, y - 1},
      {x, y + 1}
    ]
    |> Enum.filter(fn {i, j} ->
      i >= 0 and i < width and j >= 0 and j < height
    end)
  end

  def get_size(grid) do
    {
      grid
      |> Map.keys()
      |> Enum.max_by(fn {x, _} -> x end)
      |> elem(0)
      |> Kernel.+(1),
      grid
      |> Map.keys()
      |> Enum.max_by(fn {_, y} -> y end)
      |> elem(1)
      |> Kernel.+(1)
    }
  end

  def loop(grid, queue, distances, visited_nodes, is_desc) do
    case length(queue) do
      x when x > 0 -> in_loop(grid, queue, distances, visited_nodes, is_desc)
      _ -> distances
    end
  end

  def compare_heights(node_height, neighbor_height, is_desc) do
    case is_desc do
      true -> node_height - neighbor_height <= 1
      false -> neighbor_height - node_height <= 1
    end
  end

  def run_algorithm(grid, node, neighbor, {queue, distances, visited_nodes}, is_desc) do
    node_height = Map.get(grid, node, %{value: :infinity}).value

    neighbor_height = Map.get(grid, neighbor, %{value: :infinity}).value

    {queue, distances, visited_nodes} =
      case compare_heights(node_height, neighbor_height, is_desc) do
        false ->
          {queue, distances, visited_nodes}

        true ->
          neighbor_distance = Map.get(distances, neighbor, :infinity)

          new_distance = Map.fetch!(distances, node) + 1

          {queue, distances, visited_nodes} =
            if new_distance < neighbor_distance do
              {queue ++ [{neighbor, new_distance}], Map.put(distances, neighbor, new_distance),
               visited_nodes}
            else
              {queue, distances, visited_nodes}
            end

          queue =
            if !MapSet.member?(visited_nodes, neighbor) &&
                 !Enum.any?(queue, fn {node, _} -> node == neighbor end) do
              queue ++ [{neighbor, new_distance}]
            else
              queue
            end

          {queue, distances, visited_nodes}
      end

    {queue, distances, visited_nodes}
  end

  def in_loop(grid, queue, distances, visited_nodes, is_desc) do
    {node, queue} = Enum.split(queue, 1)
    node = List.first(node)

    {node, _distance} = node

    visited_nodes = MapSet.put(visited_nodes, node)

    neighbors = get_neighbors(node, get_size(grid))

    {queue, distances, visited_nodes} =
      neighbors
      |> Enum.reduce(
        {queue, distances, visited_nodes},
        fn neighbor, {queue, distances, visited_nodes} ->
          run_algorithm(grid, node, neighbor, {queue, distances, visited_nodes}, is_desc)
        end
      )

    loop(grid, queue, distances, visited_nodes, is_desc)
  end

  def part_one do
    grid =
      File.read!("input.txt")
      |> String.trim()
      |> String.split("\r\n")
      |> Enum.map(&parse_line/1)
      |> Enum.with_index()
      |> Enum.flat_map(fn {squares, y} ->
        squares
        |> Enum.with_index()
        |> Enum.map(fn {square, x} ->
          {{x, y}, square}
        end)
      end)
      |> Map.new()

    startpoint =
      grid
      |> Enum.find(fn {_position, square} ->
        square.is_startpoint
      end)
      |> elem(0)

    endpoint =
      grid
      |> Enum.find(fn {_position, square} ->
        square.is_endpoint
      end)
      |> elem(0)

    distances = %{startpoint => 0}
    unvisited_touched_nodes = [{startpoint, 0}]
    visited_nodes = MapSet.new()

    loop(grid, unvisited_touched_nodes, distances, visited_nodes, false)
    |> Map.fetch!(endpoint)
    |> IO.inspect()
  end

  def part_two do
    grid =
      File.read!("input.txt")
      |> String.trim()
      |> String.split("\r\n")
      |> Enum.map(&parse_line/1)
      |> Enum.with_index()
      |> Enum.flat_map(fn {squares, y} ->
        squares
        |> Enum.with_index()
        |> Enum.map(fn {square, x} ->
          {{x, y}, square}
        end)
      end)
      |> Map.new()

    endpoint =
      grid
      |> Enum.find(fn {_position, square} ->
        square.is_endpoint
      end)
      |> elem(0)

    distances = %{endpoint => 0}
    unvisited_touched_nodes = [{endpoint, 0}]
    visited_nodes = MapSet.new()

    distances = loop(grid, unvisited_touched_nodes, distances, visited_nodes, true)

    grid
    |> Enum.filter(fn {_, value} ->
      Map.get(value, :value, :infinity) == 0
    end)
    |> Enum.map(fn {position, _} ->
      Map.get(distances, position, :infinity)
    end)
    |> Enum.min()
    |> IO.inspect()
  end
end
