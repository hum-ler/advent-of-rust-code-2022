use std::collections::HashSet;

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-14.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: String) -> Result<usize> {
    let (mut obstacles, max_depth) = parse_input_into_rock_layout(input)?;

    Ok(drop_sand_into_void((500, 0), &mut obstacles, max_depth))
}

fn part_2(input: String) -> Result<usize> {
    let (mut obstacles, max_depth) = parse_input_into_rock_layout(input)?;

    Ok(drop_sand_onto_floor((500, 0), &mut obstacles, max_depth))
}

type Coord = (usize, usize);

/// Converts the input into the positions of all rocks, and the maximum depth.
fn parse_input_into_rock_layout(input: String) -> Result<(HashSet<Coord>, usize)> {
    let rocks = input
        .lines()
        .map(parse_line_into_rocks)
        .collect::<Result<Vec<Vec<_>>>>()?
        .into_iter()
        .flatten()
        .collect::<HashSet<_>>();

    let Some(bottom_rock) = rocks
        .clone()
        .into_iter()
        .reduce(|acc, rock| if rock.1 > acc.1 { rock } else { acc })
    else {
        return Err(anyhow!("Cannot deduce max depth"));
    };

    Ok((rocks, bottom_rock.1))
}

fn parse_line_into_rocks(line: &str) -> Result<Vec<Coord>> {
    let vertices = line
        .split_terminator(" -> ")
        .map(|coord_str| {
            let Some((x, y)) = coord_str.split_once(",") else {
                return Err(anyhow!("Cannot split rock: {}", coord_str));
            };

            Ok((x.parse()?, y.parse()?))
        })
        .collect::<Result<Vec<_>>>()?;

    // Fill in the edges.
    Ok(vertices
        .windows(2)
        .map(|pair| match (pair[0], pair[1]) {
            ((start_x, start_y), (end_x, end_y)) if start_x == end_x && start_y < end_y => {
                Ok((start_y..=end_y).map(|y| (start_x, y)).collect::<Vec<_>>())
            }
            ((start_x, start_y), (end_x, end_y)) if start_x == end_x && start_y > end_y => {
                Ok((end_y..=start_y).map(|y| (start_x, y)).collect::<Vec<_>>())
            }
            ((start_x, start_y), (end_x, end_y)) if start_y == end_y && start_x < end_x => {
                Ok((start_x..=end_x).map(|x| (x, start_y)).collect::<Vec<_>>())
            }
            ((start_x, start_y), (end_x, end_y)) if start_y == end_y && start_x > end_x => {
                Ok((end_x..=start_x).map(|x| (x, start_y)).collect::<Vec<_>>())
            }
            ((start_x, start_y), (end_x, end_y)) if start_x != end_x && start_y != end_y => Err(
                anyhow!("Consecutive vertices are not on the same vertical / horizontal line"),
            ),
            (start, end) if start == end => Err(anyhow!("Consecutive vertices are the same coord")),
            _ => unreachable!(),
        })
        .collect::<Result<Vec<Vec<_>>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>())
}

/// Simulates sand dropping with no floor.
///
/// Returns the index of the first unit of sand that falls beyond max_depth.
///
/// obstacles is updated with sand obstacles.
fn drop_sand_into_void(source: Coord, obstacles: &mut HashSet<Coord>, max_depth: usize) -> usize {
    let mut index = 0;

    'simulation: loop {
        // Initialize a new sand unit.
        let (mut x, mut y) = source;

        loop {
            // Check for termination.
            if y > max_depth {
                break 'simulation;
            }

            if obstacles.contains(&(x, y + 1))
                && obstacles.contains(&(x - 1, y + 1))
                && obstacles.contains(&(x + 1, y + 1))
            {
                // Settle.
                obstacles.insert((x, y));
                break;
            }

            // Straight-down
            if !obstacles.contains(&(x, y + 1)) {
                y += 1;
                continue;
            }

            // Bottom-left
            if !obstacles.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
                continue;
            }

            // Bottom-right
            if !obstacles.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
                continue;
            }
        }

        index += 1;
    }

    index
}

/// Simulates sand dropping with endless floor.
///
/// Returns the index of the first unit of sand that is blocked at source.
///
/// obstacles is updated with sand obstacles.
fn drop_sand_onto_floor(source: Coord, obstacles: &mut HashSet<Coord>, max_depth: usize) -> usize {
    let floor_depth = max_depth + 2;

    let mut index = 0;

    'simulation: loop {
        // Initialize a new sand unit.
        let (mut x, mut y) = source;

        loop {
            // Check for termination.
            if obstacles.contains(&source) {
                break 'simulation;
            }

            if y == floor_depth - 1
                || (obstacles.contains(&(x, y + 1))
                    && obstacles.contains(&(x - 1, y + 1))
                    && obstacles.contains(&(x + 1, y + 1)))
            {
                // Settle.
                obstacles.insert((x, y));
                break;
            }

            // Straight-down
            if !obstacles.contains(&(x, y + 1)) {
                y += 1;
                continue;
            }

            // Bottom-left
            if !obstacles.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
                continue;
            }

            // Bottom-right
            if !obstacles.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
                continue;
            }
        }

        index += 1;
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 24);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 93);

        Ok(())
    }
}
