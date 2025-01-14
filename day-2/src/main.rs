use std::cmp::Ordering;

use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-2.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<u32> {
    Ok(input
        .lines()
        .map(Round::from_str_part_1)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(Round::score)
        .sum())
}

fn part_2(input: String) -> Result<u32> {
    Ok(input
        .lines()
        .map(Round::from_str_part_2)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(Round::score)
        .sum())
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Shape::Rock, Shape::Rock) => Ordering::Equal,
            (Shape::Rock, Shape::Paper) => Ordering::Less,
            (Shape::Rock, Shape::Scissors) => Ordering::Greater,
            (Shape::Paper, Shape::Rock) => Ordering::Greater,
            (Shape::Paper, Shape::Paper) => Ordering::Equal,
            (Shape::Paper, Shape::Scissors) => Ordering::Less,
            (Shape::Scissors, Shape::Rock) => Ordering::Less,
            (Shape::Scissors, Shape::Paper) => Ordering::Greater,
            (Shape::Scissors, Shape::Scissors) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Shape {
    fn score(self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

struct Round(Shape, Shape);

impl Round {
    fn score(self) -> u32 {
        let Round(opponent, counter) = self;

        match counter.cmp(&opponent) {
            Ordering::Less => counter.score(),
            Ordering::Equal => counter.score() + 3,
            Ordering::Greater => counter.score() + 6,
        }
    }

    fn from_str_part_1(s: &str) -> Result<Self> {
        let Some((opponent, counter)) = s.split_once(" ") else {
            return Err(anyhow!("Cannot split s: {}", s));
        };

        let opponent = match opponent {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            x => return Err(anyhow!("Invalid shape: {}", x)),
        };

        let counter = match counter {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            x => return Err(anyhow!("Invalid shape: {}", x)),
        };

        Ok(Round(opponent, counter))
    }

    fn from_str_part_2(s: &str) -> Result<Self> {
        let Some((opponent, counter)) = s.split_once(" ") else {
            return Err(anyhow!("Cannot split s: {}", s));
        };

        let opponent = match opponent {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            x => return Err(anyhow!("Invalid shape: {}", x)),
        };

        let counter = match (counter, &opponent) {
            ("X", Shape::Rock) => Shape::Scissors,
            ("X", Shape::Paper) => Shape::Rock,
            ("X", Shape::Scissors) => Shape::Paper,
            ("Y", x) => *x,
            ("Z", Shape::Rock) => Shape::Paper,
            ("Z", Shape::Paper) => Shape::Scissors,
            ("Z", Shape::Scissors) => Shape::Rock,
            (x, _) => return Err(anyhow!("Invalid shape: {}", x)),
        };

        Ok(Round(opponent, counter))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
A Y
B X
C Z
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 15);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 12);

        Ok(())
    }
}
