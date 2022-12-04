use std::{collections::HashSet, fs};

fn parse_range(range: &str) -> HashSet<u32> {
    let pairs = range
        .split('-')
        .map(str::parse)
        .take(2)
        .collect::<Result<Vec<u32>, _>>()
        .expect("Invalid range");
    let (start, end) = (pairs[0], pairs[1]);
    (start..=end).collect()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let part_one = contents
        .split_whitespace()
        .filter(|p| {
            let mut pairs = p.split(',').map(parse_range);

            let p1 = pairs.next().unwrap();
            let p2 = pairs.next().unwrap();

            let intersection = p1.intersection(&p2).count();

            (intersection == p1.len()) || (intersection == p2.len())
        })
        .count();

    println!("Part one: {}", part_one);

    let part_two = contents
        .split_whitespace()
        .filter(|p| {
            let mut pairs = p.split(',').map(parse_range);

            let p1 = pairs.next().unwrap();
            let p2 = pairs.next().unwrap();

            let intersection = p1.intersection(&p2).count();

            intersection > 0
        })
        .count();

    println!("Part two: {}", part_two);
}
