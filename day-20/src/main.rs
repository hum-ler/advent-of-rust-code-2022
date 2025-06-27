use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-20.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: String) -> Result<i64> {
    let numbers = input
        .lines()
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>();

    let numbers = mix(numbers)?;

    let Some(zero_pos) = numbers.iter().position(|number| number.1 == 0) else {
        return Err(anyhow!("Cannot find position of 0"));
    };

    Ok([1000, 2000, 3000]
        .into_iter()
        .map(|pos| {
            numbers
                .iter()
                .cycle()
                .nth(pos + zero_pos)
                .ok_or(anyhow!("Cannot retrieve {}th element", pos))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|number| number.1)
        .sum())
}

fn part_2(input: String) -> Result<i64> {
    let decryption_key = 811589153i64;

    let mut numbers = input
        .lines()
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .enumerate()
        .map(|(index, number)| (index, number * decryption_key))
        .collect::<Vec<_>>();

    for _ in 0..10 {
        numbers = mix(numbers)?;
    }

    let Some(zero_pos) = numbers.iter().position(|number| number.1 == 0) else {
        return Err(anyhow!("Cannot find position of 0"));
    };

    Ok([1000, 2000, 3000]
        .into_iter()
        .map(|pos| {
            numbers
                .iter()
                .cycle()
                .nth(pos + zero_pos)
                .ok_or(anyhow!("Cannot retrieve {}th element", pos))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|number| number.1)
        .sum())
}

/// Mixes the vec numbers.
///
/// Items are tuples of the original pos + the number.
fn mix(mut numbers: Vec<(usize, i64)>) -> Result<Vec<(usize, i64)>> {
    let len = numbers.len() as i64;

    for orig_pos in 0..numbers.len() {
        let Some(curr_pos) = numbers.iter().position(|number| number.0 == orig_pos) else {
            return Err(anyhow!("Cannot locate orig_pos: {}", orig_pos));
        };

        let number = numbers[curr_pos];
        let mut swaps = number.1;

        if swaps == 0 {
            continue;
        } else if swaps >= len || swaps <= -len {
            // This behavior is not well-defined in the puzzle. What happens if a number wraps
            // around and have to hop over itself?
            swaps %= len - 1;
        }

        // Bring number to the end.
        numbers.rotate_left(curr_pos + 1);

        // Remove the number.
        numbers.pop();

        // Shift by swaps.
        if swaps > 0 {
            numbers.rotate_left(swaps.unsigned_abs() as usize);
        } else {
            numbers.rotate_right(swaps.unsigned_abs() as usize);
        }

        // Push number back in.
        numbers.push(number);
    }

    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
1
2
-3
3
-2
0
4
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 3);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 1623178306);

        Ok(())
    }
}
