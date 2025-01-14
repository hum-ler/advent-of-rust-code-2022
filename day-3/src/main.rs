use std::{collections::HashSet, hash::Hash};

use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-3.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<u32> {
    input
        .lines()
        .map(split_line_into_halves)
        .map(|pair| reduce_intersection(&pair))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(map_priority)
        .sum()
}

fn part_2(input: String) -> Result<u32> {
    input
        .lines()
        .map(|line| HashSet::from_iter(line.bytes()))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(reduce_intersection)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(map_priority)
        .sum()
}

fn split_line_into_halves(line: &str) -> Vec<HashSet<u8>> {
    let (left, right) = line.split_at(line.len() / 2);

    vec![
        HashSet::from_iter(left.bytes()),
        HashSet::from_iter(right.bytes()),
    ]
}

fn map_priority(item: u8) -> Result<u32> {
    match &item {
        item if item.is_ascii_lowercase() => Ok((item - b'a' + 1) as u32),
        item if item.is_ascii_uppercase() => Ok((item - b'A' + 27) as u32),
        x => Err(anyhow!("Invalid item: {}", x)),
    }
}

/// Finds an item that intersects every [HashSet] in a list.
fn reduce_intersection<T>(sets: &[HashSet<T>]) -> Result<T>
where
    T: Copy + Eq + Hash,
{
    let intersection = sets
        .iter()
        .cloned()
        .reduce(|acc, set| acc.intersection(&set).copied().collect::<HashSet<_>>())
        .ok_or(anyhow!("Cannot find intersection in a triplet"))?;

    Vec::from_iter(intersection)
        .first()
        .copied()
        .ok_or(anyhow!("Cannot find first element of intersection"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 157);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 70);

        Ok(())
    }
}
