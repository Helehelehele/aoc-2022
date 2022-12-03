use std::{collections::HashSet, fs};

fn get_priority(c: char) -> u8 {
    c.is_uppercase()
        .then(|| c as u8 - b'A' + 27)
        .unwrap_or_else(|| c as u8 - b'a' + 1)
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let rucksacks = contents.lines().collect::<Vec<&str>>();

    let part_one = rucksacks
        .iter()
        .map(|rucksack| {
            let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);

            [compartment1, compartment2]
                .map(|compartment| compartment.chars().collect::<HashSet<char>>())
                .into_iter()
                .reduce(|acc, curr| acc.intersection(&curr).copied().collect())
                .unwrap()
                .iter()
                .fold(0, |acc, &item| acc + get_priority(item))
        })
        .map(|count| count as u32)
        .sum::<u32>();

    println!("Part One: {}", part_one);

    let part_two = rucksacks
        .chunks(3)
        .map(|chunk| {
            chunk
                .iter()
                .map(|&rucksack| rucksack.chars().collect::<HashSet<char>>())
                .reduce(|acc, curr| acc.intersection(&curr).copied().collect())
                .unwrap()
                .into_iter()
                .fold(0, |acc, item| acc + get_priority(item))
        })
        .map(|count| count as u32)
        .sum::<u32>();

    println!("Part Two: {}", part_two);
}
