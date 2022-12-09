use std::{collections::HashSet, fs};

fn set_new_tail(new_head: &(i32, i32), current_tail: &mut (i32, i32)) {
    let (new_head_x, new_head_y) = new_head;
    let (current_tail_x, current_tail_y) = current_tail;

    let x_diff = *new_head_x - *current_tail_x;
    let y_diff = *new_head_y - *current_tail_y;

    let new_one = match (x_diff, y_diff) {
        (0, 0) | (0, -1) | (0, 1) | (-1, 0) | (-1, -1) | (-1, 1) | (1, 0) | (1, -1) | (1, 1) => {
            (*current_tail_x, *current_tail_y)
        }
        (0, -2) | (-1, -2) | (1, -2) => (*new_head_x, *new_head_y + 1),
        (0, 2) | (-1, 2) | (1, 2) => (*new_head_x, *new_head_y - 1),
        (2, 0) | (2, -1) | (2, 1) => (*new_head_x - 1, *new_head_y),
        (-2, 0) | (-2, -1) | (-2, 1) => (*new_head_x + 1, *new_head_y),
        (-2, -2) => (*new_head_x + 1, *new_head_y + 1),
        (-2, 2) => (*new_head_x + 1, *new_head_y - 1),
        (2, -2) => (*new_head_x - 1, *new_head_y + 1),
        (2, 2) => (*new_head_x - 1, *new_head_y - 1),
        _ => {
            panic!("Invalid move: ({}, {})", x_diff, y_diff);
        }
    };

    current_tail.0 = new_one.0;
    current_tail.1 = new_one.1;
}

fn get_movements(input: &str) -> Vec<(i32, i32)> {
    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");

    return contents
        .lines()
        .map(|line| {
            let (direction, times_str) = line.split_at(1);
            let times = times_str.trim().parse::<i32>().unwrap();

            return (direction, times);
        })
        .fold(vec![(0, 0)], |mut acc, (direction, times)| {
            let head = acc.last().cloned().unwrap();
            let (x, y) = head;

            match direction {
                "R" => {
                    for i in 1..=times {
                        acc.push((x + i, y));
                    }
                }
                "L" => {
                    for i in 1..=times {
                        acc.push((x - i, y));
                    }
                }
                "U" => {
                    for i in 1..=times {
                        acc.push((x, y + i));
                    }
                }
                "D" => {
                    for i in 1..=times {
                        acc.push((x, y - i));
                    }
                }
                _ => panic!("Unknown direction"),
            }

            return acc;
        });
}

fn part_one() {
    let visits = get_movements("input.txt")
        .iter()
        .fold(vec![(0, 0)], |mut acc, head| {
            let mut tail = acc.last().cloned().unwrap();

            set_new_tail(head, &mut tail);

            acc.push(tail);

            return acc;
        });

    let distinct_visits = visits
        .iter()
        .fold(HashSet::new(), |mut acc, visit| {
            acc.insert(visit);
            return acc;
        })
        .len();

    println!("Part one: {}", distinct_visits);
}

fn part_two() {
    let head_moves = get_movements("input.txt");

    let mut visits = head_moves;

    for _ in 0..9 {
        visits = visits.iter().fold(vec![(0, 0)], |mut acc, head| {
            let mut tail = acc.last().cloned().unwrap();

            set_new_tail(head, &mut tail);

            acc.push(tail);

            return acc;
        });
    }

    let distinct_visits = visits
        .iter()
        .fold(HashSet::new(), |mut acc, visit| {
            acc.insert(visit);
            return acc;
        })
        .len();

    println!("Part two: {}", distinct_visits);
}

fn main() {
    part_one();
    part_two();
}
