use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn parse_coordinates_str(s: &str) -> (i32, i32) {
    match &s
        .split(", ")
        .map(|s| s.split("=").nth(1).unwrap().parse().unwrap())
        .collect::<Vec<_>>()[..]
    {
        &[x, y, ..] => (x, y),
        _ => panic!("Invalid input"),
    }
}

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let (sensor_str, beacon_str) = line.split_once(": ").unwrap();

    (
        parse_coordinates_str(sensor_str),
        parse_coordinates_str(beacon_str),
    )
}

fn get_manhattan_distance((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let coordinates = contents.lines().map(parse_line).collect::<HashMap<_, _>>();

    let filled_coordinates = coordinates
        .iter()
        .flat_map(|(s, b)| vec![s, b])
        .collect::<Vec<_>>();

    let row = 2_000_000;

    let part_one = coordinates
        .iter()
        .fold(HashSet::new(), |mut acc, (s, b)| {
            let distance = get_manhattan_distance(s, b);
            let y_distance = (row - s.1).abs();

            if y_distance <= distance {
                for x in (s.0 - distance + y_distance)..=(s.0 + distance - y_distance) {
                    if filled_coordinates.contains(&&(x, row)) {
                        continue;
                    }

                    acc.insert((x, row));
                }
            }

            acc
        })
        .len();

    println!("Part one: {}", part_one);

    let part_two = (0..=4_000_000)
        .rev()
        .fold(None, |acc, y| {
            if acc.is_some() {
                return acc;
            }
            let mut r = coordinates
                .iter()
                .fold(HashSet::new(), |mut ranges, (sensor, beacon)| {
                    let distance = get_manhattan_distance(sensor, beacon);
                    let y_distance = (y - sensor.1).abs();

                    if y_distance < distance {
                        ranges.insert((
                            sensor.0 - distance + y_distance,
                            sensor.0 + distance - y_distance,
                        ));
                    }

                    if sensor.1 == y {
                        ranges.insert((sensor.0, sensor.0));
                    }

                    if beacon.1 == y {
                        ranges.insert((beacon.0, beacon.0));
                    }

                    ranges
                })
                .into_iter()
                .collect::<Vec<_>>();

            r.sort_by(|a, b| a.0.cmp(&b.0));

            let mut x = 0;

            while r.len() > 0 {
                let head = r.remove(0);
                if head.0 > x {
                    return Some(x as i64 * 4_000_000 + y as i64);
                }

                if head.1 >= x {
                    x = head.1 + 1;
                }
            }

            acc
        })
        .unwrap();

    println!("Part two: {:?}", part_two);
}

