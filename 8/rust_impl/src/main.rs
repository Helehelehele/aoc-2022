use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let matrix = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part_one = matrix
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, col)| {
                    if i == 0 || i == matrix.len() - 1 || j == 0 || j == row.len() - 1 {
                        return true;
                    }

                    // Check the row to the left
                    let left = row[..j].iter().all(|x| x < col);
                    // Check the row to the right
                    // Note: j + 1 is used to skip the current cell
                    let right = row[j + 1..].iter().all(|x| x < col);
                    // Check the column above
                    let up = matrix[..i].iter().all(|x| x[j] < *col);
                    // Check the column below
                    // Note: i + 1 is used to skip the current cell
                    let down = matrix[i + 1..].iter().all(|x| x[j] < *col);

                    left || right || up || down
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .filter(|&x| x)
        .count();

    println!("Part one: {}", part_one);

    let part_two = matrix
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, col)| {
                    if i == 0 || i == matrix.len() - 1 || j == 0 || j == row.len() - 1 {
                        return 0;
                    }

                    let mut left_side = row[..j].iter().collect::<Vec<_>>();
                    left_side.reverse();
                    let right_side = row[j + 1..].iter().collect::<Vec<_>>();
                    let mut up_side = matrix[..i].iter().map(|x| &x[j]).collect::<Vec<_>>();
                    up_side.reverse();
                    let down_side = matrix[i + 1..].iter().map(|x| &x[j]).collect::<Vec<_>>();

                    let left_trees = left_side.iter().take_while(|&&x| x < col).count();
                    let right_trees = right_side.iter().take_while(|&&x| x < col).count();
                    let up_trees = up_side.iter().take_while(|&&x| x < col).count();
                    let down_trees = down_side.iter().take_while(|&&x| x < col).count();

                    let left = if left_trees == left_side.len() {
                        left_trees
                    } else {
                        left_trees + 1
                    };
                    let right = if right_trees == right_side.len() {
                        right_trees
                    } else {
                        right_trees + 1
                    };
                    let up = if up_trees == up_side.len() {
                        up_trees
                    } else {
                        up_trees + 1
                    };
                    let down = if down_trees == down_side.len() {
                        down_trees
                    } else {
                        down_trees + 1
                    };

                    left * right * up * down
                })
                .collect::<Vec<usize>>()
        })
        .flatten()
        .max()
        .unwrap();

    println!("Part two: {}", part_two);
}
