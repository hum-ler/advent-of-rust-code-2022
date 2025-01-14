use std::{cmp::Ordering, str::FromStr};

use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-13.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let packet_pairs = convert_input_into_packet_pair(input)?;

    Ok(packet_pairs
        .into_iter()
        .enumerate()
        .filter_map(|(index, pair)| {
            if pair.0 < pair.1 {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum())
}

fn part_2(input: String) -> Result<usize> {
    let two_marker = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(2)])]);
    let six_marker = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(6)])]);

    let mut packets = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(PacketData::from_str)
        .collect::<Result<Vec<_>>>()?;

    packets.push(two_marker.clone());
    packets.push(six_marker.clone());

    packets.sort();

    let Some(two_marker_pos) = packets.iter().position(|packet| *packet == two_marker) else {
        return Err(anyhow!("Cannot locate [[2]] marker"));
    };
    let Some(six_marker_pos) = packets.iter().position(|packet| *packet == six_marker) else {
        return Err(anyhow!("Cannot locate [[6]] marker"));
    };

    Ok((two_marker_pos + 1) * (six_marker_pos + 1))
}

#[derive(Clone, Eq, PartialEq)]
enum PacketData {
    Integer(u8),
    List(Vec<PacketData>),
}

impl FromStr for PacketData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        assert!(s.starts_with("["));
        assert!(s.ends_with("]"));

        let mut list_stack: Vec<PacketData> = Vec::new();
        let mut current_list = PacketData::List(Vec::new());
        let mut prev_byte = 0u8; // to check for multi-digit integer

        // Get rid of the outermost [] before we begin.
        for byte in &s.as_bytes()[1..(s.len() - 1)] {
            match *byte {
                b'[' => {
                    // Open a new list.
                    list_stack.push(current_list);
                    current_list = PacketData::List(Vec::new());
                }
                b']' => {
                    // Close the current list and push into parent.
                    let temp = current_list;

                    if let Some(popped) = list_stack.pop() {
                        current_list = popped;
                    } else {
                        return Err(anyhow!("Cannot pop stack for input: {}", s));
                    };

                    if let PacketData::List(mut inner_list) = current_list {
                        inner_list.push(temp);
                        current_list = PacketData::List(inner_list);
                    } else {
                        return Err(anyhow!("Cannot extract inner list for input: {}", s));
                    }
                }
                x if x.is_ascii_digit() && prev_byte.is_ascii_digit() => {
                    // Handle multi-digit integer.
                    if let PacketData::List(mut inner_list) = current_list {
                        let Some(PacketData::Integer(higher_order)) = inner_list.pop() else {
                            return Err(anyhow!("Cannot pop inner list for input: {}", s));
                        };

                        // We are using u8, so there is risk of overflow here.
                        inner_list.push(PacketData::Integer(higher_order * 10 + (x - b'0')));

                        current_list = PacketData::List(inner_list);
                    } else {
                        return Err(anyhow!("Cannot extract inner list for input: {}", s));
                    }
                }
                x if x.is_ascii_digit() => {
                    if let PacketData::List(mut inner_list) = current_list {
                        inner_list.push(PacketData::Integer(x - b'0'));
                        current_list = PacketData::List(inner_list);
                    } else {
                        return Err(anyhow!("Cannot extract inner list for input: {}", s));
                    }
                }
                b',' => (),
                x => return Err(anyhow!("Unhandled value ({}) for input: {}", x, s)),
            }

            prev_byte = *byte;
        }

        Ok(current_list)
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PacketData::Integer(left), PacketData::Integer(right)) => left.cmp(right),
            (PacketData::Integer(left), right) => {
                // Convert left to List and then compare.
                PacketData::List(vec![PacketData::Integer(*left)]).cmp(right)
            }
            (left, PacketData::Integer(right)) => {
                // Convert right to List and then compare.
                left.cmp(&PacketData::List(vec![PacketData::Integer(*right)]))
            }
            (PacketData::List(left), PacketData::List(right)) => {
                // Actual List comparison.

                let mut left = &left[..];
                let mut right = &right[..];

                loop {
                    match (left, right) {
                        ([], []) => return Ordering::Equal,
                        ([], _) => return Ordering::Less,
                        (_, []) => return Ordering::Greater,
                        _ => {
                            let left_head = &left[0];
                            let right_head = &right[0];

                            let cmp = left_head.cmp(right_head);
                            if cmp != Ordering::Equal {
                                // Found the result.
                                return cmp;
                            } else {
                                // Move on to the next element.
                                left = &left[1..];
                                right = &right[1..];
                            }
                        }
                    }
                }
            }
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type PacketPair = (PacketData, PacketData);

fn convert_input_into_packet_pair(input: String) -> Result<Vec<PacketPair>> {
    let packets = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(PacketData::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok(packets
        .chunks(2)
        .map(|pair| (pair[0].clone(), pair[1].clone()))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 13);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 140);

        Ok(())
    }
}
