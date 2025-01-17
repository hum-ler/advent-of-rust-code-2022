use std::collections::HashSet;

use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-17.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let mut jet_pattern = input.bytes().cycle();

    let mut chamber = Chamber {
        grid: HashSet::new(),
        height: 0,
    };

    for index in 0..2022 {
        let rock = Rock::next(index);
        let mut rock_pos: Coord = if index == 0 {
            (2, 3)
        } else {
            (2, chamber.height + 4) // air gap of 2 from left wall and 3 from height
        };

        loop {
            let jet = jet_pattern
                .next()
                .ok_or(anyhow!("Cannot get next jet direction"))?;

            match jet {
                b'<' => {
                    if rock.can_move_left(rock_pos, &chamber.grid) {
                        rock_pos.0 -= 1;
                    }
                }
                b'>' => {
                    if rock.can_move_right(rock_pos, &chamber.grid) {
                        rock_pos.0 += 1;
                    }
                }
                x => return Err(anyhow!("Invalid jet direction: {}", x)),
            }

            if rock.can_drop(rock_pos, &chamber.grid) {
                rock_pos.1 -= 1;
            } else {
                break;
            }
        }

        chamber.settle(rock, rock_pos);

        if index > 0 && index % 25 == 0 {
            chamber.compact_grid();
        }
    }

    Ok(chamber.height + 1)
}

fn part_2(input: String) -> Result<usize> {
    let mut jet_pattern = input.bytes().cycle();

    let mut chamber = Chamber {
        grid: HashSet::new(),
        height: 0,
    };

    // Keep the fingerprint for the top of the chamber at index 2000.
    let mut fingerprint_at_2000: Option<u64> = None;
    let mut fingerprinted_height: Option<usize> = None;

    // The index of the final rock before stopping.
    let mut final_rock: Option<usize> = None;

    // The additional height calculated for the remaining cycles.
    let mut additional_height = 0usize;

    for index in 0..1000000000000 {
        let rock = Rock::next(index);
        let mut rock_pos: Coord = if index == 0 {
            (2, 3)
        } else {
            (2, chamber.height + 4) // air gap of 2 from left wall and 3 from height
        };

        loop {
            let jet = jet_pattern
                .next()
                .ok_or(anyhow!("Cannot get next jet direction"))?;

            match jet {
                b'<' => {
                    if rock.can_move_left(rock_pos, &chamber.grid) {
                        rock_pos.0 -= 1;
                    }
                }
                b'>' => {
                    if rock.can_move_right(rock_pos, &chamber.grid) {
                        rock_pos.0 += 1;
                    }
                }
                x => return Err(anyhow!("Invalid jet direction: {}", x)),
            }

            if rock.can_drop(rock_pos, &chamber.grid) {
                rock_pos.1 -= 1;
            } else {
                break;
            }
        }

        chamber.settle(rock, rock_pos);

        if let Some(final_rock) = final_rock {
            if index == final_rock {
                break;
            }
        }

        // Compact every 25 rocks -- unless index is from 2000 until we find the cycle, compact
        // after every rock, otherwise the fingerprint might not be correct.
        if (index > 0 && index % 25 == 0) || (index > 2000 && final_rock.is_none()) {
            chamber.compact_grid();
        }

        if index == 2000 {
            // Collect the fingerprint.
            fingerprint_at_2000 = fingerprint_top(&chamber);
            fingerprinted_height = Some(chamber.height);
        } else if index > 2000
            && final_rock.is_none()
            && fingerprint_at_2000 == fingerprint_top(&chamber)
        {
            // Found the first repeat.
            let index_gap = index - 2000;
            let height_gap = chamber.height - fingerprinted_height.unwrap_or_default();
            final_rock = Some((999999997999 % index_gap) + index);
            additional_height = height_gap * (999999997999 / index_gap - 1);
        }
    }

    Ok(chamber.height + 1 + additional_height)
}

/// (x, y) where y increases upwards.
type Coord = (usize, usize);

enum Rock {
    HorizontalLine,
    Plus,
    ReflectedL,
    VerticalLine,
    Square,
}

impl Rock {
    /// Gets the next [Rock] based on the total number of [Rock]s already spawned.
    fn next(index: usize) -> Self {
        match index % 5 {
            0 => Rock::HorizontalLine,
            1 => Rock::Plus,
            2 => Rock::ReflectedL,
            3 => Rock::VerticalLine,
            4 => Rock::Square,
            _ => unreachable!(),
        }
    }

    /// Gets the [Coord]s occupied by this [Rock].
    fn occupy(&self, bounding_box_bottom_left: Coord) -> HashSet<Coord> {
        match self {
            Rock::HorizontalLine => HashSet::from([
                bounding_box_bottom_left,
                (bounding_box_bottom_left.0 + 1, bounding_box_bottom_left.1),
                (bounding_box_bottom_left.0 + 2, bounding_box_bottom_left.1),
                (bounding_box_bottom_left.0 + 3, bounding_box_bottom_left.1),
            ]),
            Rock::Plus => HashSet::from([
                (
                    bounding_box_bottom_left.0 + 1,
                    bounding_box_bottom_left.1 + 1,
                ),
                (
                    bounding_box_bottom_left.0 + 1,
                    bounding_box_bottom_left.1 + 2,
                ),
                (
                    bounding_box_bottom_left.0 + 2,
                    bounding_box_bottom_left.1 + 1,
                ),
                (bounding_box_bottom_left.0 + 1, bounding_box_bottom_left.1),
                (bounding_box_bottom_left.0, bounding_box_bottom_left.1 + 1),
            ]),
            Rock::ReflectedL => HashSet::from([
                bounding_box_bottom_left,
                (bounding_box_bottom_left.0 + 1, bounding_box_bottom_left.1),
                (bounding_box_bottom_left.0 + 2, bounding_box_bottom_left.1),
                (
                    bounding_box_bottom_left.0 + 2,
                    bounding_box_bottom_left.1 + 1,
                ),
                (
                    bounding_box_bottom_left.0 + 2,
                    bounding_box_bottom_left.1 + 2,
                ),
            ]),
            Rock::VerticalLine => HashSet::from([
                bounding_box_bottom_left,
                (bounding_box_bottom_left.0, bounding_box_bottom_left.1 + 1),
                (bounding_box_bottom_left.0, bounding_box_bottom_left.1 + 2),
                (bounding_box_bottom_left.0, bounding_box_bottom_left.1 + 3),
            ]),
            Rock::Square => HashSet::from([
                bounding_box_bottom_left,
                (bounding_box_bottom_left.0, bounding_box_bottom_left.1 + 1),
                (
                    bounding_box_bottom_left.0 + 1,
                    bounding_box_bottom_left.1 + 1,
                ),
                (bounding_box_bottom_left.0 + 1, bounding_box_bottom_left.1),
            ]),
        }
    }

    fn can_move_left(&self, bounding_box_bottom_left: Coord, grid: &HashSet<Coord>) -> bool {
        if bounding_box_bottom_left.0 == 0 {
            return false;
        }

        match self {
            Rock::HorizontalLine => {
                !grid.contains(&(bounding_box_bottom_left.0 - 1, bounding_box_bottom_left.1))
            }
            Rock::Plus => [
                (bounding_box_bottom_left.0, bounding_box_bottom_left.1 + 2),
                (
                    bounding_box_bottom_left.0 - 1,
                    bounding_box_bottom_left.1 + 1,
                ),
                bounding_box_bottom_left,
            ]
            .iter()
            .all(|coord| !grid.contains(coord)),
            Rock::ReflectedL => [
                (
                    bounding_box_bottom_left.0 + 1,
                    bounding_box_bottom_left.1 + 2,
                ),
                (
                    bounding_box_bottom_left.0 + 1,
                    bounding_box_bottom_left.1 + 1,
                ),
                (bounding_box_bottom_left.0 - 1, bounding_box_bottom_left.1),
            ]
            .iter()
            .all(|coord| !grid.contains(coord)),
            Rock::VerticalLine => [
                (
                    bounding_box_bottom_left.0 - 1,
                    bounding_box_bottom_left.1 + 3,
                ),
                (
                    bounding_box_bottom_left.0 - 1,
                    bounding_box_bottom_left.1 + 2,
                ),
                (
                    bounding_box_bottom_left.0 - 1,
                    bounding_box_bottom_left.1 + 1,
                ),
                (bounding_box_bottom_left.0 - 1, bounding_box_bottom_left.1),
            ]
            .iter()
            .all(|coord| !grid.contains(coord)),
            Rock::Square => [
                (
                    bounding_box_bottom_left.0 - 1,
                    bounding_box_bottom_left.1 + 1,
                ),
                (bounding_box_bottom_left.0 - 1, bounding_box_bottom_left.1),
            ]
            .iter()
            .all(|coord| !grid.contains(coord)),
        }
    }

    fn can_move_right(&self, bounding_box_bottom_left: Coord, grid: &HashSet<Coord>) -> bool {
        match self {
            Rock::HorizontalLine => {
                if bounding_box_bottom_left.0 + 3 == 6 {
                    return false;
                }

                !grid.contains(&(bounding_box_bottom_left.0 + 4, bounding_box_bottom_left.1))
            }
            Rock::Plus => {
                if bounding_box_bottom_left.0 + 2 == 6 {
                    return false;
                }

                [
                    (
                        bounding_box_bottom_left.0 + 2,
                        bounding_box_bottom_left.1 + 2,
                    ),
                    (
                        bounding_box_bottom_left.0 + 3,
                        bounding_box_bottom_left.1 + 1,
                    ),
                    (bounding_box_bottom_left.0 + 2, bounding_box_bottom_left.1),
                ]
                .iter()
                .all(|coord| !grid.contains(coord))
            }
            Rock::ReflectedL => {
                if bounding_box_bottom_left.0 + 2 == 6 {
                    return false;
                }

                [
                    (
                        bounding_box_bottom_left.0 + 3,
                        bounding_box_bottom_left.1 + 2,
                    ),
                    (
                        bounding_box_bottom_left.0 + 3,
                        bounding_box_bottom_left.1 + 1,
                    ),
                    (bounding_box_bottom_left.0 + 3, bounding_box_bottom_left.1),
                ]
                .iter()
                .all(|coord| !grid.contains(coord))
            }
            Rock::VerticalLine => {
                if bounding_box_bottom_left.0 == 6 {
                    return false;
                }

                [
                    (
                        bounding_box_bottom_left.0 + 1,
                        bounding_box_bottom_left.1 + 3,
                    ),
                    (
                        bounding_box_bottom_left.0 + 1,
                        bounding_box_bottom_left.1 + 2,
                    ),
                    (
                        bounding_box_bottom_left.0 + 1,
                        bounding_box_bottom_left.1 + 1,
                    ),
                    (bounding_box_bottom_left.0 + 1, bounding_box_bottom_left.1),
                ]
                .iter()
                .all(|coord| !grid.contains(coord))
            }
            Rock::Square => {
                if bounding_box_bottom_left.0 + 1 == 6 {
                    return false;
                }

                [
                    (
                        bounding_box_bottom_left.0 + 2,
                        bounding_box_bottom_left.1 + 1,
                    ),
                    (bounding_box_bottom_left.0 + 2, bounding_box_bottom_left.1),
                ]
                .iter()
                .all(|coord| !grid.contains(coord))
            }
        }
    }

    fn can_drop(&self, bounding_box_bottom_left: Coord, grid: &HashSet<Coord>) -> bool {
        if bounding_box_bottom_left.1 == 0 {
            return false;
        }

        match self {
            Rock::HorizontalLine => [
                (bounding_box_bottom_left.0, bounding_box_bottom_left.1 - 1),
                (
                    bounding_box_bottom_left.0 + 1,
                    bounding_box_bottom_left.1 - 1,
                ),
                (
                    bounding_box_bottom_left.0 + 2,
                    bounding_box_bottom_left.1 - 1,
                ),
                (
                    bounding_box_bottom_left.0 + 3,
                    bounding_box_bottom_left.1 - 1,
                ),
            ]
            .iter()
            .all(|coord| !grid.contains(coord)),
            Rock::Plus => [
                bounding_box_bottom_left,
                (
                    bounding_box_bottom_left.0 + 1,
                    bounding_box_bottom_left.1 - 1,
                ),
                (bounding_box_bottom_left.0 + 2, bounding_box_bottom_left.1),
            ]
            .iter()
            .all(|coord| !grid.contains(coord)),
            Rock::ReflectedL => [
                (bounding_box_bottom_left.0, bounding_box_bottom_left.1 - 1),
                (
                    bounding_box_bottom_left.0 + 1,
                    bounding_box_bottom_left.1 - 1,
                ),
                (
                    bounding_box_bottom_left.0 + 2,
                    bounding_box_bottom_left.1 - 1,
                ),
            ]
            .iter()
            .all(|coord| !grid.contains(coord)),
            Rock::VerticalLine => {
                !grid.contains(&(bounding_box_bottom_left.0, bounding_box_bottom_left.1 - 1))
            }
            Rock::Square => [
                (bounding_box_bottom_left.0, bounding_box_bottom_left.1 - 1),
                (
                    bounding_box_bottom_left.0 + 1,
                    bounding_box_bottom_left.1 - 1,
                ),
            ]
            .iter()
            .all(|coord| !grid.contains(coord)),
        }
    }
}

struct Chamber {
    grid: HashSet<Coord>,

    /// The height of the column.
    height: usize,
}

impl Chamber {
    /// Updates both the grid and the height with the settled [Rock].
    fn settle(&mut self, rock: Rock, bounding_box_bottom_left: Coord) {
        let occupied_coords = rock.occupy(bounding_box_bottom_left);
        self.grid = self
            .grid
            .union(&occupied_coords)
            .copied()
            .collect::<HashSet<_>>();

        let rock_height = occupied_coords
            .iter()
            .max_by_key(|coord| coord.1)
            .unwrap_or(&(0, 0))
            .1;
        if rock_height > self.height {
            self.height = rock_height;
        }
    }

    fn compact_grid(&mut self) {
        // Get the first Coord at max height.
        let Some(start) = self.grid.iter().find(|coord| coord.1 == self.height) else {
            return;
        };

        let mut top_rocks: HashSet<Coord> = HashSet::from([*start]);

        // Start by facing south, hug the left wall and collect all the [Coord]s until the [Coord]
        // ahead has col > 6.

        let mut pos = *start;
        let mut facing = Facing::South;

        while let Some((new_pos, new_facing)) = hug_left_wall(pos, facing, &self.grid) {
            top_rocks.insert(new_pos);
            pos = new_pos;
            facing = new_facing;
        }

        // Go back to max height. Start by facing south, hug the right wall until the [Coord] ahead
        // has col < 0.

        let mut pos = *start;
        let mut facing = Facing::South;

        while let Some((new_pos, new_facing)) = hug_right_wall(pos, facing, &self.grid) {
            top_rocks.insert(new_pos);
            pos = new_pos;
            facing = new_facing;
        }

        self.grid = top_rocks;
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Facing {
    North,
    East,
    South,
    West,
}

fn turn_left(facing: Facing) -> Facing {
    match facing {
        Facing::North => Facing::West,
        Facing::East => Facing::North,
        Facing::South => Facing::East,
        Facing::West => Facing::South,
    }
}

fn turn_right(facing: Facing) -> Facing {
    match facing {
        Facing::North => Facing::East,
        Facing::East => Facing::South,
        Facing::South => Facing::West,
        Facing::West => Facing::North,
    }
}

fn get_left_wall(pos: Coord, facing: Facing, grid: &HashSet<Coord>) -> Option<Coord> {
    match facing {
        Facing::North => {
            if pos.0 == 0 {
                return None;
            }

            if grid.contains(&(pos.0 - 1, pos.1)) {
                Some((pos.0 - 1, pos.1))
            } else {
                None
            }
        }
        Facing::East => {
            if grid.contains(&(pos.0, pos.1 + 1)) {
                Some((pos.0, pos.1 + 1))
            } else {
                None
            }
        }
        Facing::South => {
            if pos.0 == 6 {
                return None;
            }

            if grid.contains(&(pos.0 + 1, pos.1)) {
                Some((pos.0 + 1, pos.1))
            } else {
                None
            }
        }
        Facing::West => {
            if pos.1 == 0 {
                return None;
            }

            if grid.contains(&(pos.0, pos.1 - 1)) {
                Some((pos.0, pos.1 - 1))
            } else {
                None
            }
        }
    }
}

fn get_right_wall(pos: Coord, facing: Facing, grid: &HashSet<Coord>) -> Option<Coord> {
    match facing {
        Facing::North => {
            if pos.0 == 6 {
                return None;
            }

            if grid.contains(&(pos.0 + 1, pos.1)) {
                Some((pos.0 + 1, pos.1))
            } else {
                None
            }
        }
        Facing::East => {
            if pos.1 == 0 {
                return None;
            }

            if grid.contains(&(pos.0, pos.1 - 1)) {
                Some((pos.0, pos.1 - 1))
            } else {
                None
            }
        }
        Facing::South => {
            if pos.0 == 0 {
                return None;
            }

            if grid.contains(&(pos.0 - 1, pos.1)) {
                Some((pos.0 - 1, pos.1))
            } else {
                None
            }
        }
        Facing::West => {
            if grid.contains(&(pos.0, pos.1 + 1)) {
                Some((pos.0, pos.1 + 1))
            } else {
                None
            }
        }
    }
}

fn hug_left_wall(pos: Coord, facing: Facing, grid: &HashSet<Coord>) -> Option<(Coord, Facing)> {
    // Check if we have hit the chamber wall itself.
    if facing == Facing::South && pos.0 == 6 {
        return None;
    }

    if let Some(left_pos) = get_left_wall(pos, facing, grid) {
        Some((left_pos, turn_left(facing)))
    } else {
        Some((pos, turn_right(facing)))
    }
}

fn hug_right_wall(pos: Coord, facing: Facing, grid: &HashSet<Coord>) -> Option<(Coord, Facing)> {
    // Check if we have hit the chamber wall itself.
    if facing == Facing::South && pos.0 == 0 {
        return None;
    }

    if let Some(right_pos) = get_right_wall(pos, facing, grid) {
        Some((right_pos, turn_right(facing)))
    } else {
        Some((pos, turn_left(facing)))
    }
}

fn fingerprint_top(chamber: &Chamber) -> Option<u64> {
    if chamber.height < 7 {
        return None;
    }

    let mut fingerprint = 0u64;

    for y in ((chamber.height - 7)..=chamber.height).rev() {
        let mut row = 0u64;

        for x in 0..7 {
            row <<= 1;
            if chamber.grid.contains(&(x, y)) {
                row += 1;
            }
        }

        fingerprint <<= 8;
        fingerprint += row;
    }

    Some(fingerprint)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.to_string())?, 3068);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 1514285714288);

        Ok(())
    }
}
