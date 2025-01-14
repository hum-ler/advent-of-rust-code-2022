use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-10.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<i32> {
    let operands = parse_input_into_operands(input)?;

    let register = cumulate_operands_into_register(operands);

    Ok(signal_strength(&register))
}

fn part_2(input: String) -> Result<String> {
    let operands = parse_input_into_operands(input)?;

    let register = cumulate_operands_into_register(operands);

    let screen = print_crt(&register);
    println!("{}", screen);

    Ok(screen)
}

/// Converts input into operands to be applied at each cycle.
fn parse_input_into_operands(input: String) -> Result<Vec<Option<i32>>> {
    // Init to 1. This also makes operands take effect only at the following cycle.
    let mut operands: Vec<Option<i32>> = vec![Some(1)];

    for line in input.lines() {
        match line {
            "noop" => operands.push(None),
            addx if addx.starts_with("addx ") => {
                let Some((_, value)) = addx.split_once(" ") else {
                    return Err(anyhow!("Cannot split addx: {}", addx));
                };

                // 2 cycles.
                operands.push(None);
                operands.push(Some(value.parse::<i32>()?));
            }
            x => return Err(anyhow!("Invalid line: {}", x)),
        }
    }

    Ok(operands)
}

/// Generates the cumulative register value at each cycle.
fn cumulate_operands_into_register(operands: Vec<Option<i32>>) -> Vec<i32> {
    operands.iter().fold(Vec::new(), |mut acc, operand| {
        let cumulative_value = if acc.is_empty() {
            0
        } else {
            acc[acc.len() - 1]
        };

        if let Some(operand) = operand {
            acc.push(cumulative_value + operand);
        } else {
            acc.push(cumulative_value);
        }

        acc
    })
}

fn signal_strength(register: &[i32]) -> i32 {
    // Cycle starts at 1, so index at -1.
    register[19] * 20
        + register[59] * 60
        + register[99] * 100
        + register[139] * 140
        + register[179] * 180
        + register[219] * 220
}

fn print_crt(register: &[i32]) -> String {
    let mut screen = String::new();

    for (index, value) in register.iter().take(240).enumerate() {
        if ((*value - 1)..=(*value + 1)).contains(&(index as i32 % 40)) {
            screen.push('#');
        } else {
            screen.push('.');
        }

        // Add eol unless it is the last line.
        if index != 239 && index % 40 == 39 {
            screen.push('\n');
        }
    }

    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 13140);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(
            part_2(EXAMPLE_INPUT.trim().to_string())?,
            String::from(
                r"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
                .trim()
            )
        );

        Ok(())
    }
}
