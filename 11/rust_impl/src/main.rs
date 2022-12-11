use std::fs;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    if_true: usize,
    if_false: usize,
    inspection_count: u64,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
    Double,
}

#[derive(Debug, Copy, Clone)]
enum Test {
    DivideBy(u64),
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let original = contents
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(|chunk| {
            let monkey = chunk
                .iter()
                .skip(1)
                .take(5)
                .map(|l| l.trim())
                .collect::<Vec<&str>>();

            let items = monkey[0]
                .split(":")
                .map(|s| s.trim())
                .nth(1)
                .unwrap()
                .split(",")
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let operation_tokens = monkey[1]
                .split("=")
                .map(|s| s.trim())
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|s| s.trim())
                .skip(1) // Skip "old"
                .collect::<Vec<&str>>();

            let operation = match &operation_tokens[..] {
                ["+", "old"] => Operation::Double,
                ["*", "old"] => Operation::Square,
                ["+", x] if x.parse::<u64>().is_ok() => Operation::Add(x.parse::<u64>().unwrap()),

                ["*", x] if x.parse::<u64>().is_ok() => {
                    Operation::Multiply(x.parse::<u64>().unwrap())
                }
                _ => panic!("Invalid operation"),
            };

            let test_token = monkey[2]
                .split(":")
                .map(|s| s.trim())
                .nth(1)
                .unwrap()
                .split_whitespace()
                .skip(2)
                .next()
                .unwrap()
                .trim()
                .parse::<u64>()
                .unwrap();

            let test = Test::DivideBy(test_token);

            let if_true = monkey[3]
                .split(":")
                .map(|s| s.trim())
                .nth(1)
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let if_false = monkey[4]
                .split(":")
                .map(|s| s.trim())
                .nth(1)
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            return Monkey {
                items,
                operation,
                test,
                if_true,
                if_false,
                inspection_count: 0,
            };
        })
        .collect::<Vec<Monkey>>();

    // Part One
    let mut monkeys = original.clone();
    let monkey_count = monkeys.len().clone();

    for _ in 0..20 {
        for i in 0..monkey_count {
            let current_monkey_items = monkeys[i].items.clone();
            let current_monkey_operation = monkeys[i].operation.clone();
            let current_monkey_test = monkeys[i].test.clone();
            let current_monkey_if_true = monkeys[i].if_true.clone();
            let current_monkey_if_false = monkeys[i].if_false.clone();

            for item in current_monkey_items {
                let operation_result = match current_monkey_operation {
                    Operation::Add(x) => item + x,
                    Operation::Multiply(x) => item * x,
                    Operation::Square => item * item,
                    Operation::Double => item * 2,
                };

                let worry_level: u64 = operation_result / 3;

                let test_result = match current_monkey_test {
                    Test::DivideBy(x) => worry_level % x == 0,
                };

                if test_result {
                    monkeys[current_monkey_if_true].items.push(worry_level);
                } else {
                    monkeys[current_monkey_if_false].items.push(worry_level);
                }

                monkeys[i].inspection_count += 1;
            }

            monkeys[i].items.clear();
        }
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

    let part_one = monkeys
        .iter()
        .take(2)
        .fold(1, |acc, m| acc * m.inspection_count);

    println!("{:?}", part_one);

    // Part Two

    // Trick for LCM
    let divisor = original.iter().fold(1, |acc, m| {
        acc * match m.test {
            Test::DivideBy(x) => x,
        }
    });
    let mut monkeys = original;

    for _ in 0..10000 {
        for i in 0..monkey_count {
            let current_monkey_items = monkeys[i].items.clone();
            let current_monkey_operation = monkeys[i].operation.clone();
            let current_monkey_test = monkeys[i].test.clone();
            let current_monkey_if_true = monkeys[i].if_true.clone();
            let current_monkey_if_false = monkeys[i].if_false.clone();

            for item in current_monkey_items {
                let operation_result = match current_monkey_operation {
                    Operation::Add(x) => item + x,
                    Operation::Multiply(x) => item * x,
                    Operation::Square => item * item,
                    Operation::Double => item * 2,
                };

                let worry_level: u64 = operation_result % divisor;

                let test_result = match current_monkey_test {
                    Test::DivideBy(x) => worry_level % x == 0,
                };

                if test_result {
                    monkeys[current_monkey_if_true].items.push(worry_level);
                } else {
                    monkeys[current_monkey_if_false].items.push(worry_level);
                }

                monkeys[i].inspection_count += 1;
            }

            monkeys[i].items.clear();
        }
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

    let part_two = monkeys
        .iter()
        .take(2)
        .fold(1, |acc, m| acc * m.inspection_count);

    println!("{:?}", part_two);
}
