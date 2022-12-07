use std::{collections::HashMap, fs};

#[derive(Debug, Default)]
struct Node {
    children: HashMap<String, Node>,
    size: usize,
}

fn calculate_sizes(node: &mut Node) -> usize {
    if node.size > 0 {
        return node.size;
    }

    node.size = node
        .children
        .values_mut()
        .map(|child| {
            if child.children.len() > 0 {
                calculate_sizes(child)
            } else {
                child.size
            }
        })
        .sum::<usize>();

    node.size
}

// fn print_dirs(node: &Node, name: &str, depth: usize) {
//     println!("{}- {}: {}", " ".repeat(depth * 2), name, node.size);
//
//     let mut keys = node.children.keys().cloned().collect::<Vec<_>>();
//     keys.sort();
//
//     for key in keys {
//         print_dirs(node.children.get(&key).unwrap(), &key, depth + 1);
//     }
// }

fn get_dirs_under_limit(node: &Node, limit: usize, acc: &mut Vec<usize>) {
    node.children
        .values()
        .for_each(|child| get_dirs_under_limit(child, limit, acc));

    if node.children.len() > 0 && node.size <= limit {
        acc.push(node.size)
    }
}

fn get_least_dir_over_limit(node: &Node, limit: usize) -> Option<usize> {
    if node.children.len() == 0 {
        return None;
    }

    let min_child_size = node
        .children
        .values()
        .map(|child| get_least_dir_over_limit(child, limit))
        .filter_map(|x| x)
        .filter(|&x| x >= limit)
        .min();

    if node.size < limit {
        min_child_size
    } else {
        min_child_size.or(Some(node.size))
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let initial: (Node, Vec<&str>) = (Node::default(), Vec::new());

    let mut root = contents
        .lines()
        .skip(1)
        .fold(initial, |mut acc, line| {
            if line == "$ cd /" {
                acc.1.clear();
                return acc;
            }

            if line == "$ cd .." && acc.1.len() > 0 {
                acc.1.pop();
                return acc;
            }

            if line.starts_with("$ cd") {
                acc.1.push(&line[5..]);
                return acc;
            }

            if line == "$ ls" || line.starts_with("dir") {
                return acc;
            }

            let words = line.split_whitespace().take(2).collect::<Vec<_>>();
            let (size, name) = (words[0].parse::<usize>().unwrap(), words[1]);

            let mut current = &mut acc.0;

            for dir in &acc.1 {
                current = current.children.entry(dir.to_string()).or_default();
            }

            current.children.entry(name.to_string()).or_default().size = size;

            return acc;
        })
        .0;

    calculate_sizes(&mut root);
    // print_dirs(&root, "/", 0);

    let mut part_one: Vec<usize> = Vec::new();

    get_dirs_under_limit(&root, 100000, &mut part_one);

    println!("Part one: {:?}", part_one.iter().sum::<usize>());

    let part_two_limit = root.size - 40000000;
    let part_two = get_least_dir_over_limit(&root, part_two_limit).unwrap();

    println!("Part two: {:?}", part_two);
}
