use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-1.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<u32> {
    input
        .split_terminator("\n\n")
        .map(|section| {
            Ok(section
                .lines()
                .map(str::parse::<u32>)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .sum())
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .max()
        .ok_or(anyhow!("Cannot find max overall"))
}

fn part_2(input: String) -> Result<u32> {
    let mut calories = input
        .split_terminator("\n\n")
        .map(|section| {
            Ok(section
                .lines()
                .map(str::parse::<u32>)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .sum::<u32>())
        })
        .collect::<Result<Vec<_>>>()?;

    calories.sort_by(|a, b| b.cmp(a)); // descending

    Ok(calories.into_iter().take(3).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 24000);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 45000);

        Ok(())
    }
}
