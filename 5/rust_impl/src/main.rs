use std::{collections::VecDeque, fs, str};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut lines = contents.lines();

    let container_matrix = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let (_ /*ids_line*/, container_lines) = container_matrix.split_last().unwrap();

    // let col_count = ids_line.split_whitespace().count();
    let containers = container_lines
        .iter()
        .map(|&line| {
            line.as_bytes()
                .chunks(4)
                .map(str::from_utf8)
                .map(Result::unwrap)
                .map(|s| {
                    s.trim()
                        .replace(&['[', ']', ' '], "")
                        .chars()
                        .collect::<Vec<_>>()
                })
                .map(|s| VecDeque::from_iter(s))
                .collect::<Vec<_>>()
        })
        .reduce(|mut acc, next| {
            // Build deques in the following format:
            // [bottom, ..., top]
            for (i, ni) in next.iter().enumerate() {
                if ni.len() > 0 {
                    acc[i].push_front(ni[0]);
                }
            }
            acc
        })
        .unwrap();

    let mut part_one = containers.clone();
    let mut part_two = containers.clone();

    // Moves
    lines.take_while(|line| !line.is_empty()).for_each(|line| {
        let [amount, source, to]: [usize; 3] = line
            .split_whitespace()
            .filter_map(|word| word.parse::<usize>().ok())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let src_index = source - 1;
        let to_index = to - 1;
        let part_one_drain_index = part_one[src_index].len() - amount;

        // Drain the source containers and extend the destination containers
        let part_one_drained = part_one[src_index]
            .drain(part_one_drain_index..)
            .rev()
            .collect::<Vec<_>>();

        part_one[to_index].extend(part_one_drained);

        let part_two_drain_index = part_two[src_index].len() - amount;

        let part_two_drained = part_two[src_index]
            .drain(part_two_drain_index..)
            .collect::<Vec<_>>();

        part_two[to_index].extend(part_two_drained);
    });

    let part_one_result = part_one
        .iter()
        .map(|container| container.back().unwrap_or(&' '))
        .collect::<String>();

    println!("Part One: {}", part_one_result);

    let part_two_result = part_two
        .iter()
        .map(|container| container.back().unwrap_or(&' '))
        .collect::<String>();

    println!("Part Two: {}", part_two_result);
}
