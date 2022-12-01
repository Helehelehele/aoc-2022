use std::cmp;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let elves = contents.split("\r\n\r\n").collect::<Vec<&str>>();

    let mut elves_with_cals = elves
        .iter()
        .enumerate()
        .map(|(idx, elf)| {
            let calories = elf
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>();

            return (idx, calories);
        })
        .collect::<Vec<(usize, i32)>>();

    elves_with_cals.sort_by_key(|x| cmp::Reverse(x.1));

    let top_elves = elves_with_cals
        .iter()
        .take(3)
        .collect::<Vec<&(usize, i32)>>();

    println!(
        "Elf {} has the most calories with {}",
        top_elves[0].0 + 1,
        top_elves[0].1
    );

    let total_calories = top_elves.iter().map(|x| x.1).sum::<i32>();

    println!("Total calories: {}", total_calories);
}
