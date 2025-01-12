use std::hash::Hash;

use anyhow::{anyhow, Result};
use itertools::Itertools;

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-6.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let mut marker = None;

    let datastream = input.bytes().collect::<Vec<_>>();
    for (col, _) in datastream.iter().enumerate() {
        if col < 3 {
            continue;
        }

        if all_unique(&datastream[(col - 3)..=col]) {
            marker = Some(col + 1);
            break;
        }
    }

    marker.ok_or(anyhow!("Cannot find marker in input: {}", input))
}

fn part_2(input: String) -> Result<usize> {
    let mut marker = None;

    let datastream = input.bytes().collect::<Vec<_>>();
    for (col, _) in datastream.iter().enumerate() {
        if col < 13 {
            continue;
        }

        if all_unique(&datastream[(col - 13)..=col]) {
            marker = Some(col + 1);
            break;
        }
    }

    marker.ok_or(anyhow!("Cannot find marker in input: {}", input))
}

/// Checks that the entire slice is composed of unique elements.
fn all_unique<T>(slice: &[T]) -> bool
where
    T: Eq + Hash,
{
    slice.iter().unique().count() == slice.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1a() -> Result<()> {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_1(String::from(input))?, 7);

        Ok(())
    }

    #[test]
    fn example_1b() -> Result<()> {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part_1(String::from(input))?, 5);

        Ok(())
    }

    #[test]
    fn example_1c() -> Result<()> {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part_1(String::from(input))?, 6);

        Ok(())
    }

    #[test]
    fn example_1d() -> Result<()> {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_1(String::from(input))?, 10);

        Ok(())
    }

    #[test]
    fn example_1e() -> Result<()> {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part_1(String::from(input))?, 11);

        Ok(())
    }

    #[test]
    fn example_2a() -> Result<()> {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_2(String::from(input))?, 19);

        Ok(())
    }

    #[test]
    fn example_2b() -> Result<()> {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part_2(String::from(input))?, 23);

        Ok(())
    }

    #[test]
    fn example_2c() -> Result<()> {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part_2(String::from(input))?, 23);

        Ok(())
    }

    #[test]
    fn example_2d() -> Result<()> {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_2(String::from(input))?, 29);

        Ok(())
    }

    #[test]
    fn example_2e() -> Result<()> {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part_2(String::from(input))?, 26);

        Ok(())
    }
}
