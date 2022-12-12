use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

fn parse_line(line: &str) -> (Vec<u8>, Option<usize>, Option<usize>) {
    let mut starting_index: Option<usize> = None;
    let mut ending_index: Option<usize> = None;
    let squares = line
        .chars()
        .enumerate()
        .map(|(idx, c)| match c {
            lower if lower.is_lowercase() => lower as u8 - 'a' as u8,
            'S' => {
                starting_index = Some(idx);
                0
            }
            'E' => {
                ending_index = Some(idx);
                'z' as u8 - 'a' as u8
            }
            _ => panic!("Invalid character"),
        })
        .collect::<Vec<u8>>();

    return (squares, starting_index, ending_index);
}

fn get_neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < width - 1 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < height - 1 {
        neighbors.push((x, y + 1));
    }

    return neighbors;
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let lines = contents
        .lines()
        .map(parse_line)
        .collect::<Vec<(Vec<u8>, Option<usize>, Option<usize>)>>();

    let grid: HashMap<(usize, usize), u8> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, (squares, _, _))| {
            squares
                .iter()
                .enumerate()
                .map(move |(x, square)| ((x, y), *square))
        })
        .collect();

    let width = lines[0].0.len();
    let height = lines.len();

    let starting_index = lines
        .iter()
        .enumerate()
        .find_map(|(y, (_, starting_index, _))| starting_index.map(|x| (x, y)))
        .unwrap();

    let ending_index = lines
        .iter()
        .enumerate()
        .find_map(|(y, (_, _, ending_index))| ending_index.map(|x| (x, y)))
        .unwrap();

    // s/o Dijkstra
    let mut distances = HashMap::new();
    distances.insert(starting_index, 0);

    let mut unvisited_touched_nodes = BinaryHeap::new();
    unvisited_touched_nodes.push(starting_index);

    let mut visited_nodes = HashSet::new();

    while let Some((x, y)) = unvisited_touched_nodes.pop() {
        visited_nodes.insert((x, y));

        for (neighbor_x, neighbor_y) in get_neighbors(x, y, width, height) {
            let height_difference = grid[&(neighbor_x, neighbor_y)] as i32 - grid[&(x, y)] as i32;
            if height_difference > 1 {
                continue;
            }

            let neigbor_distance = distances
                .entry((neighbor_x, neighbor_y))
                .or_insert(std::u32::MAX)
                .clone();

            let new_distance = distances.get(&(x, y)).unwrap() + 1;
            if new_distance < neigbor_distance {
                distances.insert((neighbor_x, neighbor_y), new_distance);
                unvisited_touched_nodes.push((neighbor_x, neighbor_y));
            }

            if !visited_nodes.contains(&(neighbor_x, neighbor_y))
                && !unvisited_touched_nodes
                    .iter()
                    .any(|(x, y)| *x == neighbor_x && *y == neighbor_y)
            {
                unvisited_touched_nodes.push((neighbor_x, neighbor_y));
            }
        }
    }

    println!("Part one: {}", distances.get(&ending_index).unwrap());

    distances.clear();
    unvisited_touched_nodes.clear();
    visited_nodes.clear();

    distances.insert(ending_index, 0);
    unvisited_touched_nodes.push(ending_index);

    while let Some((x, y)) = unvisited_touched_nodes.pop() {
        visited_nodes.insert((x, y));

        for (neighbor_x, neighbor_y) in get_neighbors(x, y, width, height) {
            let height_difference = grid[&(x, y)] as i32 - grid[&(neighbor_x, neighbor_y)] as i32;
            if height_difference > 1 {
                continue;
            }

            let neigbor_distance = distances
                .entry((neighbor_x, neighbor_y))
                .or_insert(std::u32::MAX)
                .clone();

            let new_distance = distances.get(&(x, y)).unwrap() + 1;
            if new_distance < neigbor_distance {
                distances.insert((neighbor_x, neighbor_y), new_distance);
                unvisited_touched_nodes.push((neighbor_x, neighbor_y));
            }

            if !visited_nodes.contains(&(neighbor_x, neighbor_y))
                && !unvisited_touched_nodes
                    .iter()
                    .any(|(x, y)| *x == neighbor_x && *y == neighbor_y)
            {
                unvisited_touched_nodes.push((neighbor_x, neighbor_y));
            }
        }
    }

    let part_two = grid
        .iter()
        .filter(|(_, height)| **height == 0)
        .map(|((x, y), _)| distances.get(&(*x, *y)).or(Some(&std::u32::MAX)).unwrap())
        .min()
        .unwrap();

    println!("Part two: {}", part_two);
}
