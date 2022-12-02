use std::fs;

const OPPONENTS_SHAPES: [&'static str; 3] = ["A", "B", "C"]; // Rock, Paper, Scissors
const MY_SHAPES: [&'static str; 3] = ["X", "Y", "Z"]; // Rock, Paper, Scissors OR Lose, Draw, Win

fn get_round_result(opponents_shape: &str, my_shape: &str) -> u32 {
    let opponent_index = OPPONENTS_SHAPES
        .iter()
        .position(|&shape| shape == opponents_shape)
        .unwrap() as u32;

    let my_index = MY_SHAPES
        .iter()
        .position(|&shape| shape == my_shape)
        .unwrap() as u32;

    let result = (opponent_index + 3 - my_index) % 3;

    let my_shape_score: u32 = my_index + 1;

    return match result {
        0 => 3 + my_shape_score, // Tie
        1 => my_shape_score,     // Lose
        2 => 6 + my_shape_score, // Win
        _ => panic!("Invalid result"),
    };
}

fn get_my_play(opponents_shape: &str, result: &str) -> &'static str {
    let opponent_index = OPPONENTS_SHAPES
        .iter()
        .position(|&shape| shape == opponents_shape)
        .unwrap() as u32;

    let result_index = MY_SHAPES.iter().position(|&shape| shape == result).unwrap() as u32;

    let my_index = match result_index {
        0 => (opponent_index + 2) % 3, // Lose
        1 => opponent_index,           // Draw
        2 => (opponent_index + 1) % 3, // Win
        _ => panic!("Invalid result"),
    };

    return MY_SHAPES[my_index as usize];
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let rounds = contents
        .lines()
        .map(|round| {
            let plays = round.split_whitespace().collect::<Vec<&str>>();
            let (opponent, me) = match plays[..] {
                [opponent, me] => (opponent, me),
                _ => panic!("Invalid input"),
            };

            return (opponent, me);
        })
        .collect::<Vec<(&str, &str)>>();

    let total_misunderstanding = rounds
        .iter()
        .map(|(opponent, me)| get_round_result(&opponent, &me))
        .sum::<u32>();

    println!("Total misunderstanding: {}", total_misunderstanding);

    let actual_score = rounds
        .iter()
        .map(|(opponent, result)| {
            let my_play = get_my_play(opponent, result);
            return get_round_result(opponent, my_play);
        })
        .sum::<u32>();

    println!("Actual score: {}", actual_score);
}
