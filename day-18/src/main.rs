use std::collections::HashSet;

use anyhow::{anyhow, Result};
use itertools::Itertools;

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-18.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let coords = parse_input_into_coords(input)?;

    Ok(coords.len() * 6 - adjacent_pairs(&coords) * 2)
}

fn part_2(input: String) -> Result<usize> {
    let coords = parse_input_into_coords(input)?;

    count_surfaces(&coords)
}

type Coord = (u8, u8, u8);

fn parse_input_into_coords(input: String) -> Result<HashSet<Coord>> {
    input
        .lines()
        .map(|line| {
            let coord_vec = line
                .split_terminator(",")
                .map(str::parse::<u8>)
                .collect::<Result<Vec<_>, _>>()?;

            Ok((coord_vec[0], coord_vec[1], coord_vec[2]))
        })
        .collect()
}

fn adjacent_pairs(coords: &HashSet<Coord>) -> usize {
    coords
        .iter()
        .tuple_combinations()
        .filter(|(first, second)| {
            (first.0.abs_diff(second.0) == 1 && first.1 == second.1 && first.2 == second.2)
                || (first.0 == second.0 && first.1.abs_diff(second.1) == 1 && first.2 == second.2)
                || (first.0 == second.0 && first.1 == second.1 && first.2.abs_diff(second.2) == 1)
        })
        .count()
}

fn bounding_box(coords: &HashSet<Coord>) -> (Coord, Coord) {
    coords.iter().fold(
        ((u8::MAX, u8::MAX, u8::MAX), (0, 0, 0)),
        |mut acc, coord| {
            if coord.0 < acc.0 .0 {
                acc.0 .0 = coord.0;
            }
            if coord.1 < acc.0 .1 {
                acc.0 .1 = coord.1;
            }
            if coord.2 < acc.0 .2 {
                acc.0 .2 = coord.2;
            }

            if coord.0 > acc.1 .0 {
                acc.1 .0 = coord.0;
            }
            if coord.1 > acc.1 .1 {
                acc.1 .1 = coord.1;
            }
            if coord.2 > acc.1 .2 {
                acc.1 .2 = coord.2;
            }

            acc
        },
    )
}

/// Expands the bounding box by 1.
fn expand_bounding_box(bounding_box: (Coord, Coord)) -> Result<(Coord, Coord)> {
    if bounding_box.0 .0 == 0 || bounding_box.0 .1 == 0 || bounding_box.0 .2 == 0 {
        return Err(anyhow!("Lower bound contains 0"));
    }

    Ok((
        (
            bounding_box.0 .0 - 1,
            bounding_box.0 .1 - 1,
            bounding_box.0 .2 - 1,
        ),
        (
            bounding_box.1 .0 + 1,
            bounding_box.1 .1 + 1,
            bounding_box.1 .2 + 1,
        ),
    ))
}

/// Translates all [Coord]s (only in the positive direction).
fn translate(coords: &HashSet<Coord>, translation: (u8, u8, u8)) -> HashSet<Coord> {
    coords
        .iter()
        .map(|coord| {
            (
                coord.0 + translation.0,
                coord.1 + translation.1,
                coord.2 + translation.2,
            )
        })
        .collect()
}

fn count_surfaces(coords: &HashSet<Coord>) -> Result<usize> {
    let mut coords = coords.clone();

    // Shift all the coords if the 0-planes are not clear.
    let (lower_bound, _) = bounding_box(&coords);
    let translation = (
        (lower_bound.0 != 1) as u8,
        (lower_bound.1 != 1) as u8,
        (lower_bound.2 != 1) as u8,
    );
    if translation != (0, 0, 0) {
        coords = translate(&coords, translation);
    }

    let (lower_bound, upper_bound) = expand_bounding_box(bounding_box(&coords))?;

    let mut done = HashSet::new();
    Ok(flood_search(
        lower_bound,
        &coords,
        lower_bound,
        upper_bound,
        &mut done,
    ))
}

fn flood_search(
    start: Coord,
    lava: &HashSet<Coord>,
    lower_bound: Coord,
    upper_bound: Coord,
    done: &mut HashSet<Coord>,
) -> usize {
    if done.contains(&start) {
        return 0;
    }

    done.insert(start);

    let mut surfaces_found = 0;
    let mut search_stack = Vec::new();

    let (x, y, z) = start;

    if x != lower_bound.0 {
        if lava.contains(&(x - 1, y, z)) {
            surfaces_found += 1;
        } else {
            search_stack.push((x - 1, y, z));
        }
    }
    if x != upper_bound.0 {
        if lava.contains(&(x + 1, y, z)) {
            surfaces_found += 1;
        } else {
            search_stack.push((x + 1, y, z));
        }
    }

    if y != lower_bound.1 {
        if lava.contains(&(x, y - 1, z)) {
            surfaces_found += 1;
        } else {
            search_stack.push((x, y - 1, z));
        }
    }
    if y != upper_bound.1 {
        if lava.contains(&(x, y + 1, z)) {
            surfaces_found += 1;
        } else {
            search_stack.push((x, y + 1, z));
        }
    }

    if z != lower_bound.2 {
        if lava.contains(&(x, y, z - 1)) {
            surfaces_found += 1;
        } else {
            search_stack.push((x, y, z - 1));
        }
    }
    if z != upper_bound.2 {
        if lava.contains(&(x, y, z + 1)) {
            surfaces_found += 1;
        } else {
            search_stack.push((x, y, z + 1));
        }
    }

    surfaces_found
        + search_stack
            .into_iter()
            .map(|coord| flood_search(coord, lava, lower_bound, upper_bound, done))
            .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 64);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 58);

        Ok(())
    }
}
