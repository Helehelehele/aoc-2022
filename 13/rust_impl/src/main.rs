use std::fs;

#[derive(Debug, PartialEq)]
enum Packet {
    Array(Vec<Packet>),
    Integer(u32),
}

fn parse_packet(packet: &str) -> Packet {
    let split_packet = packet.chars().collect::<Vec<char>>();

    match &split_packet[..] {
        [num] if num.is_digit(10) => Packet::Integer(num.to_digit(10).unwrap()),
        [num @ ..] if num.iter().all(|c| c.is_digit(10)) => {
            let joined = num.iter().collect::<String>();

            Packet::Integer(joined.parse::<u32>().unwrap())
        }
        ['[', ']'] => Packet::Array(vec![]),
        ['[', rest @ .., ']'] => {
            let mut packets = Vec::new();
            let mut bracket_level = 0;

            let mut current_packet = String::new();

            for c in rest {
                match c {
                    '[' => {
                        bracket_level += 1;
                        current_packet.push(*c);
                    }
                    ']' => {
                        bracket_level -= 1;
                        current_packet.push(*c);
                    }
                    ',' if bracket_level == 0 => {
                        packets.push(parse_packet(&current_packet));
                        current_packet = String::new();
                    }
                    _ => current_packet.push(*c),
                }
            }

            if !current_packet.is_empty() {
                packets.push(parse_packet(&current_packet));
            }

            Packet::Array(packets)
        }
        _ => panic!("Invalid packet: {}", packet),
    }
}

fn compare_packets(left: &Packet, right: &Packet) -> std::cmp::Ordering {
    match (left, right) {
        (Packet::Integer(l), Packet::Integer(r)) => l.cmp(r),
        (Packet::Array(l), Packet::Array(r)) => {
            for (l, r) in l.iter().zip(r.iter()) {
                match compare_packets(l, r) {
                    std::cmp::Ordering::Equal => continue,
                    other => return other,
                }
            }

            if l.len() < r.len() {
                std::cmp::Ordering::Less
            } else if l.len() > r.len() {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
        (Packet::Integer(l), Packet::Array(_)) => {
            let wrapped = Packet::Array(vec![Packet::Integer(*l)]);
            compare_packets(&wrapped, right)
        }
        (Packet::Array(_), Packet::Integer(r)) => {
            let wrapped = Packet::Array(vec![Packet::Integer(*r)]);
            compare_packets(left, &wrapped)
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let parsed = contents
        .split("\r\n\r\n")
        .map(|pair| {
            pair.lines()
                .map(|packet| parse_packet(packet))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part_one = parsed
        .iter()
        .enumerate()
        .map(|(i, pair)| {
            let (left, right) = (&pair[0], &pair[1]);
            let cmp = compare_packets(left, right);

            (i + 1, cmp == std::cmp::Ordering::Less)
        })
        .filter(|(_, left_wins)| *left_wins)
        .map(|(i, _)| i)
        .sum::<usize>();

    println!("Part one: {}", part_one);

    let divider_packets = vec![
        Packet::Array(vec![Packet::Array(vec![Packet::Integer(2)])]),
        Packet::Array(vec![Packet::Array(vec![Packet::Integer(6)])]),
    ];

    let mut part_two = parsed
        .iter()
        .flatten()
        .chain(divider_packets.iter())
        .collect::<Vec<_>>();

    part_two.sort_by(|left, right| compare_packets(left, right));

    let part_two_product = part_two
        .iter()
        .enumerate()
        .map(|(idx, packet)| (idx + 1, packet))
        .filter(|(_, packet)| divider_packets.contains(packet))
        .map(|(idx, _)| idx)
        .product::<usize>();

    println!("Part two: {:?}", part_two_product);
}
