use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-25.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<String> {
    dec_to_snafu(input.lines().map(snafu_to_dec).sum::<Result<i64>>()?)
}

fn part_2(_input: String) -> Result<()> {
    Err(anyhow!("No part 2"))
}

fn snafu_to_dec(value: &str) -> Result<i64> {
    value
        .bytes()
        .rev()
        .enumerate()
        .map(|(index, byte)| {
            let multiplier = 5i64.pow(index as u32);

            Ok(match byte {
                b'2' => 2 * multiplier,
                b'1' => multiplier,
                b'0' => 0,
                b'-' => -multiplier,
                b'=' => -2 * multiplier,
                x => return Err(anyhow!("Invalid byte: {}", x)),
            })
        })
        .sum()
}

fn dec_to_snafu(mut value: i64) -> Result<String> {
    let mut output = Vec::new();

    while value != 0 {
        let remainder = value % 5;
        value /= 5;

        match remainder {
            4 => {
                value += 1; // carry over
                output.push(b'-');
            }
            3 => {
                value += 1; // carry over
                output.push(b'=');
            }
            2 => output.push(b'2'),
            1 => output.push(b'1'),
            0 => output.push(b'0'),
            _ => unreachable!(),
        }
    }

    output.reverse();

    String::from_utf8(output).map_err(|error| anyhow!("Cannot map from u8: {}", error))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(
            part_1(EXAMPLE_INPUT.trim().to_string())?,
            String::from("2=-1=0")
        );

        Ok(())
    }
}
