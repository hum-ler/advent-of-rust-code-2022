use std::collections::{HashMap, HashSet};

use anyhow::Result;

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-23.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let mut elves = parse_input_into_elves(input);

    for round in 0..10 {
        elves = conduct_round(round, elves);
    }

    let (top_left, bottom_right) = bounding_box(&elves);

    Ok((bottom_right.0 - top_left.0 + 1) * (bottom_right.1 - top_left.1 + 1) - elves.len())
}

fn part_2(input: String) -> Result<usize> {
    let mut elves = parse_input_into_elves(input);

    let mut round = 0usize;
    loop {
        let prev_elves = elves.clone();

        elves = conduct_round(round, elves);

        if elves == prev_elves {
            break;
        }

        round += 1;
    }

    Ok(round + 1)
}

type Coord = (usize, usize);

fn parse_input_into_elves(input: String) -> HashSet<Coord> {
    let padding = 1001; // handle 1000 rounds

    let mut elves = HashSet::new();

    for (row, line) in input.lines().enumerate() {
        for (col, byte) in line.bytes().enumerate() {
            if byte == b'#' {
                elves.insert((row + padding, col + padding));
            }
        }
    }

    elves
}

fn conduct_round(round: usize, mut elves: HashSet<Coord>) -> HashSet<Coord> {
    let direction_order = [round % 4, (round + 1) % 4, (round + 2) % 4, (round + 3) % 4];

    // Gather proposals.
    let mut proposals: HashMap<Coord, Vec<Coord>> = HashMap::new();
    for &(row, col) in &elves {
        if all_directions_clear((row, col), &elves) {
            continue;
        }

        for direction in direction_order {
            match direction {
                // n
                0 => {
                    if n_clear((row, col), &elves) {
                        proposals
                            .entry((row - 1, col))
                            .and_modify(|proposers| proposers.push((row, col)))
                            .or_default()
                            .push((row, col));

                        break;
                    }
                }

                // s
                1 => {
                    if s_clear((row, col), &elves) {
                        proposals
                            .entry((row + 1, col))
                            .and_modify(|proposers| proposers.push((row, col)))
                            .or_default()
                            .push((row, col));

                        break;
                    }
                }

                // w
                2 => {
                    if w_clear((row, col), &elves) {
                        proposals
                            .entry((row, col - 1))
                            .and_modify(|proposers| proposers.push((row, col)))
                            .or_default()
                            .push((row, col));

                        break;
                    }
                }

                // e
                3 => {
                    if e_clear((row, col), &elves) {
                        proposals
                            .entry((row, col + 1))
                            .and_modify(|proposers| proposers.push((row, col)))
                            .or_default()
                            .push((row, col));

                        break;
                    }
                }

                _ => unreachable!(),
            }
        }
    }

    // Resolve proposals.
    proposals.into_iter().for_each(|(proposal, proposers)| {
        if proposers.len() == 1 {
            elves.remove(&proposers[0]);
            elves.insert(proposal);
        }
    });

    elves
}

fn all_directions_clear(elf: Coord, elves: &HashSet<Coord>) -> bool {
    let (row, col) = elf;

    n_clear(elf, elves) // n, ne, nw
        && s_clear(elf, elves) // s, se, sw
        && !elves.contains(&(row, col - 1)) // w
        && !elves.contains(&(row, col + 1)) // e
}

fn n_clear(elf: Coord, elves: &HashSet<Coord>) -> bool {
    let (row, col) = elf;

    !elves.contains(&(row - 1, col))
        && !elves.contains(&(row - 1, col + 1))
        && !elves.contains(&(row - 1, col - 1))
}

fn s_clear(elf: Coord, elves: &HashSet<Coord>) -> bool {
    let (row, col) = elf;

    !elves.contains(&(row + 1, col))
        && !elves.contains(&(row + 1, col + 1))
        && !elves.contains(&(row + 1, col - 1))
}

fn w_clear(elf: Coord, elves: &HashSet<Coord>) -> bool {
    let (row, col) = elf;

    !elves.contains(&(row, col - 1))
        && !elves.contains(&(row - 1, col - 1))
        && !elves.contains(&(row + 1, col - 1))
}

fn e_clear(elf: Coord, elves: &HashSet<Coord>) -> bool {
    let (row, col) = elf;

    !elves.contains(&(row, col + 1))
        && !elves.contains(&(row - 1, col + 1))
        && !elves.contains(&(row + 1, col + 1))
}

fn bounding_box(elves: &HashSet<Coord>) -> (Coord, Coord) {
    elves.iter().fold(
        ((usize::MAX, usize::MAX), (usize::MIN, usize::MIN)),
        |mut acc, elf| {
            if elf.0 < acc.0 .0 {
                acc.0 .0 = elf.0;
            }
            if elf.0 > acc.1 .0 {
                acc.1 .0 = elf.0;
            }
            if elf.1 < acc.0 .1 {
                acc.0 .1 = elf.1;
            }
            if elf.1 > acc.1 .1 {
                acc.1 .1 = elf.1;
            }

            acc
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 110);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 20);

        Ok(())
    }
}
