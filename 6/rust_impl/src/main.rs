use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let char_indices = contents.char_indices().collect::<Vec<_>>();

    let part_one = char_indices
        .windows(4)
        .filter(|&w| w.iter().map(|(_, c)| c).collect::<HashSet<_>>().len() == 4)
        .map(|w| w[3].0 + 1)
        .next()
        .unwrap();

    println!("Part one: {:?}", part_one);

    let part_two = char_indices
        .windows(14)
        .filter(|&w| w.iter().map(|(_, c)| c).collect::<HashSet<_>>().len() == 14)
        .map(|w| w[13].0 + 1)
        .next()
        .unwrap();

    println!("Part two: {:?}", part_two);
}
