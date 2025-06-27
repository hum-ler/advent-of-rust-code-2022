use std::ops::RangeInclusive;

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-4.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: String) -> Result<usize> {
    Ok(input
        .lines()
        .map(parse_line_into_ranges)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|(first, second)| {
            (first.contains(second.start()) && first.contains(second.end()))
                || (second.contains(first.start()) && second.contains(first.end()))
        })
        .count())
}

fn part_2(input: String) -> Result<usize> {
    Ok(input
        .lines()
        .map(parse_line_into_ranges)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|(first, second)| first.contains(second.start()) || second.contains(first.start()))
        .count())
}

fn parse_line_into_ranges(line: &str) -> Result<(RangeInclusive<u8>, RangeInclusive<u8>)> {
    let Some((first, second)) = line.split_once(",") else {
        return Err(anyhow!("Cannot split line into ranges: {}", line));
    };

    let Some((first_left, first_right)) = first.split_once("-") else {
        return Err(anyhow!("Cannot split range into boundaries: {}", first));
    };

    let Some((second_left, second_right)) = second.split_once("-") else {
        return Err(anyhow!("Cannot split range into boundaries: {}", second));
    };

    Ok((
        (first_left.parse()?..=first_right.parse()?),
        (second_left.parse()?..=second_right.parse()?),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 2);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 4);

        Ok(())
    }
}
