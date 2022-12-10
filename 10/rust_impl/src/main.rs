use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let desired_indices: [usize; 6] = [20, 60, 100, 140, 180, 220].map(|x| x - 1);

    let part_one = contents
        .lines()
        .fold(vec![], |mut acc, line| {
            let mut tokens = line.split_whitespace().fuse();
            let first = tokens.next();
            let second = tokens.next();
            match first {
                Some("noop") => acc.push(0),
                Some("addx") => match second {
                    Some(num) => match num.parse::<i32>() {
                        Ok(n) => acc.extend([0, n]),
                        Err(_) => panic!("Invalid number: {}", num),
                    },
                    _ => panic!("addx requires a number"),
                },
                None => panic!("Empty line"),
                Some(instruction) => panic!("Invalid instruction: {}", instruction),
            }

            return acc;
        })
        .iter()
        .fold(vec![], |mut acc, x| {
            let head = acc.last().cloned().unwrap_or(1);
            acc.push(head + x);
            acc
        })
        .iter()
        .enumerate()
        .map(|(idx, val)| (idx + 1, val))
        .filter(|(i, _)| desired_indices.contains(i))
        .fold(0, |acc, (idx, val)| acc + ((idx as i32 + 1) * val));

    println!("Part one: {:?}", part_one);

    // Part two
    contents
        .lines()
        .fold(vec![], |mut acc, line| {
            let mut tokens = line.split_whitespace().fuse();
            let first = tokens.next();
            let second = tokens.next();
            match first {
                Some("noop") => acc.push(0),
                Some("addx") => match second {
                    Some(num) => match num.parse::<i32>() {
                        Ok(n) => acc.extend([0, n]),
                        Err(_) => panic!("Invalid number: {}", num),
                    },
                    _ => panic!("addx requires a number"),
                },
                None => panic!("Empty line"),
                Some(instruction) => panic!("Invalid instruction: {}", instruction),
            }

            return acc;
        })
        .iter()
        .enumerate()
        .fold((vec![vec![" "; 40]; 6], 1 as i32), |acc, (idx, val)| {
            let (dx, dy) = (idx % 40, (idx as f32 / 40.0).floor() as usize);
            let (mut grid, mut x) = acc;

            grid = match dx as i32 {
                should_paint
                    if should_paint == x - 1 || should_paint == x || should_paint == x + 1 =>
                {
                    grid[dy][dx] = "█";
                    grid
                }
                _ => {
                    grid[dy][dx] = " ";
                    grid
                }
            };

            x += val;

            return (grid, x);
        })
        .0
        .iter()
        .map(|row| row.join(""))
        .for_each(|row| println!("{}", row));
}
