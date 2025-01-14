use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};
use regex::Regex;

fn main() {
    match get_part("inputs/day-5.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<String> {
    let (mut stacks, steps) = convert_input_into_stacks_steps(input)?;

    for step in steps {
        let (from, to, size) = step;

        for _ in 0..size {
            let Some(top) = stacks[from].pop() else {
                return Err(anyhow!("Cannot pop from stack: {}", from));
            };

            stacks[to].push(top);
        }
    }

    let tops = stacks
        .into_iter()
        .map(|stack| stack.into_iter().last().ok_or(anyhow!("Cannot get top")))
        .collect::<Result<Vec<_>>>()?;

    Ok(String::from_utf8(tops)?)
}

fn part_2(input: String) -> Result<String> {
    let (mut stacks, steps) = convert_input_into_stacks_steps(input)?;

    for step in steps {
        let (from, to, size) = step;

        let to_keep = stacks[from].len() - size;
        let to_move = Vec::from(&stacks[from][to_keep..]);

        stacks[to].extend_from_slice(&to_move);
        stacks[from].truncate(to_keep);
    }

    let tops = stacks
        .into_iter()
        .map(|stack| stack.into_iter().last().ok_or(anyhow!("Cannot get top")))
        .collect::<Result<Vec<_>>>()?;

    Ok(String::from_utf8(tops)?)
}

type Stacks = Vec<Vec<u8>>;

/// Vec<(from, to, size)>
type Steps = Vec<(usize, usize, usize)>;

fn convert_input_into_stacks_steps(input: String) -> Result<(Stacks, Steps)> {
    let Some((stacks_part, steps_part)) = input.split_once("\n\n") else {
        return Err(anyhow!(
            "Cannot split input into stacks and steps: {}",
            input
        ));
    };

    Ok((
        convert_input_into_stacks(stacks_part)?,
        convert_input_into_steps(steps_part)?,
    ))
}

fn convert_input_into_stacks(input: &str) -> Result<Stacks> {
    let mut stacks: Vec<Vec<u8>> = Vec::new();

    let mut lines = input.lines().collect::<Vec<_>>();

    // Get indices.
    let Some(indices) = lines.pop() else {
        return Err(anyhow!("Cannot pull last line from input"));
    };
    let stack_count = indices
        .split_whitespace()
        .filter(|token| !token.is_empty())
        .last()
        .map(str::parse::<u8>)
        .ok_or(anyhow!("Cannot get stack count"))?
        .map_err(|error| anyhow!("Cannot parse stack count: {}", error))?;

    // Initialize the stacks.
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    // Do each row from bottom up.
    while let Some(line) = lines.pop() {
        line.bytes().enumerate().for_each(|(col, byte)| {
            if col < 1 {
                return;
            }

            match byte {
                b'[' | b']' | b' ' => (),
                x => stacks[(col - 1) / 4].push(x),
            }
        });
    }

    Ok(stacks)
}

fn convert_input_into_steps(input: &str) -> Result<Steps> {
    let regex = Regex::new(r"^move (?<size>\d+) from (?<from>\d+) to (?<to>\d+)$")?;

    input
        .lines()
        .map(|line| {
            let Some(captures) = regex.captures(line) else {
                return Err(anyhow!("Cannot capture from line: {}", line));
            };

            Ok((
                captures["from"].parse::<usize>()? - 1,
                captures["to"].parse::<usize>()? - 1,
                captures["size"].parse()?,
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(
            part_1(
                EXAMPLE_INPUT
                    .trim_start_matches("\n")
                    .trim_end()
                    .to_string()
            )?,
            String::from("CMZ")
        );

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(
            part_2(
                EXAMPLE_INPUT
                    .trim_start_matches("\n")
                    .trim_end()
                    .to_string()
            )?,
            String::from("MCD")
        );

        Ok(())
    }
}
