defmodule DaySeven do
  def build_dir_tree do
    File.read!("input.txt")
    |> String.split("\r\n")
    |> Enum.filter(&(&1 != ""))
    |> Enum.reduce(
      %{
        root: %{
          children: %{},
          size: 0
        },
        current_path: []
      },
      fn line, acc ->
        case line do
          "$ cd /" ->
            %{acc | current_path: []}

          "$ cd .." ->
            %{acc | current_path: Enum.drop(acc.current_path, -1)}

          "$ cd " <> dir ->
            %{acc | current_path: acc.current_path ++ [dir]}

          "$ ls" ->
            acc

          "dir " <> name ->
            update_in(
              acc,
              [:root, :children] ++
                (acc.current_path
                 |> Enum.map(&([&1] ++ [:children]))
                 |> List.flatten()),
              fn dir ->
                if Map.has_key?(dir, name) do
                  dir
                else
                  put_in(dir, [Access.key(name, %{})], %{
                    children: %{},
                    size: 0
                  })
                end
              end
            )

          _ ->
            [size, name] = String.split(line, " ")

            acc.current_path
            |> Enum.with_index()
            |> Enum.map(fn {_, idx} -> acc.current_path |> Enum.take(idx + 1) end)
            |> Enum.map(fn path ->
              path |> Enum.map(fn p -> [:children, p] end) |> List.flatten()
            end)
            |> (fn paths -> if Enum.count(paths) == 0, do: [[]], else: [[]] ++ paths end).()
            |> Enum.map(fn path ->
              [:root] ++ path ++ [:size]
            end)
            |> Enum.reduce(acc, fn path, a ->
              update_in(a, path, &(&1 + String.to_integer(size)))
            end)
            |> update_in(
              [:root, :children] ++
                (acc.current_path
                 |> Enum.map(&([&1] ++ [:children]))
                 |> List.flatten()),
              fn dir ->
                put_in(dir, [Access.key(name, %{})], %{
                  children: %{},
                  size: String.to_integer(size)
                })
              end
            )
        end
      end
    )
    |> Map.get(:root)
  end

  def get_dirs_under_limit(node, limit) do
    node
    |> Map.get(:children)
    |> Enum.map(fn {_, child} -> get_dirs_under_limit(child, limit) end)
    |> List.flatten()
    |> (fn children_sizes ->
          if node.size <= limit && map_size(node[:children]) > 0,
            do: [node.size] ++ children_sizes,
            else: children_sizes
        end).()
  end

  def part_one do
    build_dir_tree()
    |> get_dirs_under_limit(100_000)
    |> Enum.sum()
    |> IO.inspect()
  end

  def get_least_dir_over_limit(node, limit) do
    node
    |> Map.get(:children)
    |> Enum.filter(fn {_, child} -> map_size(child[:children]) > 0 end)
    |> Enum.map(fn {_, child} -> get_least_dir_over_limit(child, limit) end)
    |> Enum.filter(fn result -> result != :not_found end)
    |> Enum.min_by(fn size -> size end, fn -> :not_found end)
    |> (fn
          :not_found ->
            if node.size >= limit, do: node.size, else: :not_found

          result ->
            result
        end).()
  end

  def part_two do
    build_dir_tree()
    |> (fn root -> {root, root[:size] - 40_000_000} end).()
    |> (fn {root, limit} -> get_least_dir_over_limit(root, limit) end).()
    |> IO.inspect()
  end
end
