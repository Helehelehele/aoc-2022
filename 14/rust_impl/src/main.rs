use std::{fs, ops::Add};

fn parse_rocks(line: &str) -> Vec<(u32, u32)> {
    line.split(" -> ")
        .map(|s| {
            let parsed_coordinates = s
                .split(",")
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .take(2)
                .collect::<Vec<u32>>();

            match &parsed_coordinates[..] {
                [x, y] => (*x, *y),
                _ => panic!("Invalid coordinates"),
            }
        })
        .collect::<Vec<(u32, u32)>>()
}

fn add_rocks_in_between(rocks: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    rocks.iter().fold(vec![], |acc, (x, y)| {
        let last = acc.last();

        if last.is_none() {
            return vec![(*x, *y)];
        }

        let (last_x, last_y) = last.unwrap();

        if last_x == x {
            let mut new_acc = acc.clone();

            let y_diff = *y as i32 - *last_y as i32;

            for i in 1..y_diff.abs() {
                let new_y = if y_diff > 0 {
                    last_y + i as u32
                } else {
                    last_y - i as u32
                };

                new_acc.push((*x, new_y));
            }

            new_acc.push((*x, *y));

            new_acc
        } else if last_y == y {
            let mut new_acc = acc.clone();

            let x_diff = *x as i32 - *last_x as i32;

            for i in 1..x_diff.abs() {
                let new_x = if x_diff > 0 {
                    last_x + i as u32
                } else {
                    last_x - i as u32
                };

                new_acc.push((new_x, *y));
            }

            new_acc.push((*x, *y));

            new_acc
        } else {
            panic!("Invalid coordinates");
        }
    })
}

fn part_one() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let scan = contents
        .lines()
        .map(parse_rocks)
        .map(add_rocks_in_between)
        .collect::<Vec<Vec<(u32, u32)>>>();

    let sand_source = (500, 0);

    let min_x = scan
        .iter()
        .map(|rocks| rocks.iter().map(|(x, _)| x).min().unwrap())
        .min()
        .unwrap()
        .min(&sand_source.0);

    let max_x = scan
        .iter()
        .map(|rocks| rocks.iter().map(|(x, _)| x).max().unwrap())
        .max()
        .unwrap()
        .max(&sand_source.0);

    let min_y = scan
        .iter()
        .map(|rocks| rocks.iter().map(|(_, y)| y).min().unwrap())
        .min()
        .unwrap()
        .min(&sand_source.1);

    let max_y = scan
        .iter()
        .map(|rocks| rocks.iter().map(|(_, y)| y).max().unwrap())
        .max()
        .unwrap()
        .max(&sand_source.1);

    let mut map = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

    for rocks in scan.iter() {
        for (x, y) in rocks {
            map[(y - min_y) as usize][(x - min_x) as usize] = '#';
        }
    }

    map[(sand_source.1 - min_y) as usize][(sand_source.0 - min_x) as usize] = '+';

    let mut active_sand = (
        sand_source.0 as i32 - *min_x as i32,
        sand_source.1 as i32 - *min_y as i32,
    );
    let mut total_sand = 0;

    loop {
        let (x, y) = active_sand;

        if y == *max_y as i32 {
            break;
        }

        if x <= 0 || x > *max_x as i32 - *min_x as i32 {
            break;
        }

        if map[y as usize + 1][x as usize] == '.' {
            active_sand = (x, y + 1);
        } else if map[y as usize + 1][x as usize - 1] == '.' {
            active_sand = (x - 1, y + 1);
        } else if map[y as usize + 1][x as usize + 1] == '.' {
            active_sand = (x + 1, y + 1);
        } else {
            map[y as usize][x as usize] = 'o';
            total_sand += 1;
            active_sand = (
                sand_source.0 as i32 - *min_x as i32,
                sand_source.1 as i32 - *min_y as i32,
            );
        }
    }

    for row in map.iter() {
        println!("{}", row.iter().collect::<String>());
    }

    println!("Total sand: {}", total_sand);
}

fn part_two() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let scan = contents
        .lines()
        .map(parse_rocks)
        .map(add_rocks_in_between)
        .map(|rocks| {
            rocks
                .iter()
                .map(|(x, y)| (*x as i32, *y as i32))
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<Vec<(i32, i32)>>>();

    let sand_source: (i32, i32) = (500, 0);

    let mut max_y = scan
        .iter()
        .map(|rocks| rocks.iter().map(|(_, y)| y).max().unwrap())
        .max()
        .unwrap()
        .max(&sand_source.1)
        .add(1);

    let min_x = (sand_source.0 - max_y) - 1;
    let max_x = (sand_source.0 + max_y) + 1;

    let mut map = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y + 1) as usize];

    for rocks in scan.iter() {
        for (x, y) in rocks {
            map[*y as usize][(x - min_x) as usize] = '#';
        }
    }

    // Add floor
    map.push(vec!['#'; (max_x - min_x + 1) as usize]);
    max_y += 1;

    let mut active_sand = (sand_source.0 - min_x, sand_source.1);
    let mut total_sand = 0;

    loop {
        let (x, y) = active_sand;

        if y == max_y {
            break;
        }

        if x < 0 || x > max_x - min_x {
            break;
        }

        if map[y as usize + 1][x as usize] == '.' {
            active_sand = (x, y + 1);
        } else if map[y as usize + 1][x as usize - 1] == '.' {
            active_sand = (x - 1, y + 1);
        } else if map[y as usize + 1][x as usize + 1] == '.' {
            active_sand = (x + 1, y + 1);
        } else {
            map[y as usize][x as usize] = 'o';
            total_sand += 1;

            if active_sand == (sand_source.0 - min_x, sand_source.1) {
                break;
            }

            active_sand = (sand_source.0 - min_x, sand_source.1);
        }
    }

    for row in map.iter() {
        println!("{}", row.iter().collect::<String>());
    }

    println!("Total sand: {}", total_sand);
}

fn main() {
    // part_one();
    part_two();
}
