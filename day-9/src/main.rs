use std::collections::HashSet;

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-9.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: String) -> Result<usize> {
    let mut coords: Vec<Coord> = vec![(0, 0); 2];

    let mut trail: HashSet<Coord> = HashSet::new();
    trail.insert(coords[coords.len() - 1]);

    input.lines().try_for_each(|line| {
        perform_move(line, &mut coords, &mut trail)?;

        Ok::<_, anyhow::Error>(())
    })?;

    Ok(trail.len())
}

fn part_2(input: String) -> Result<usize> {
    let mut coords: Vec<Coord> = vec![(0, 0); 10];

    let mut trail: HashSet<Coord> = HashSet::new();
    trail.insert(coords[coords.len() - 1]);

    input.lines().try_for_each(|line| {
        perform_move(line, &mut coords, &mut trail)?;

        Ok::<_, anyhow::Error>(())
    })?;

    Ok(trail.len())
}

type Coord = (isize, isize);

fn move_up(coord: Coord) -> Coord {
    let (row, col) = coord;

    (row - 1, col)
}

fn move_right(coord: Coord) -> Coord {
    let (row, col) = coord;

    (row, col + 1)
}

fn move_down(coord: Coord) -> Coord {
    let (row, col) = coord;

    (row + 1, col)
}

fn move_left(coord: Coord) -> Coord {
    let (row, col) = coord;

    (row, col - 1)
}

/// Pulls follower knot towards leader knot.
///
/// Returns the new [Coord] of the tail.
fn tug(leader: Coord, follower: Coord) -> Coord {
    let (leader_row, leader_col) = leader;
    let (mut follower_row, mut follower_col) = follower;

    // ne
    if follower_row == leader_row + 2 && follower_col == leader_col - 2 {
        follower_row -= 1;
        follower_col += 1;
    }

    // se
    if follower_row == leader_row - 2 && follower_col == leader_col - 2 {
        follower_row += 1;
        follower_col += 1;
    }

    // sw
    if follower_row == leader_row - 2 && follower_col == leader_col + 2 {
        follower_row += 1;
        follower_col -= 1;
    }

    // nw
    if follower_row == leader_row + 2 && follower_col == leader_col + 2 {
        follower_row -= 1;
        follower_col -= 1;
    }

    // n
    if follower_row == leader_row + 2 {
        follower_row -= 1;
        follower_col = leader_col;
    }

    // e
    if follower_col == leader_col - 2 {
        follower_col += 1;
        follower_row = leader_row;
    }

    // s
    if follower_row == leader_row - 2 {
        follower_row += 1;
        follower_col = leader_col;
    }

    // w
    if follower_col == leader_col + 2 {
        follower_col -= 1;
        follower_row = leader_row;
    }

    (follower_row, follower_col)
}

fn perform_move(input: &str, rope: &mut [Coord], trail: &mut HashSet<Coord>) -> Result<()> {
    assert!(rope.len() >= 2);

    let Some((direction, steps)) = input.split_once(" ") else {
        return Err(anyhow!("Cannot split input: {}", input));
    };

    let steps = steps.parse::<usize>()?;

    (0..steps).try_for_each(|_| {
        match direction {
            "U" => rope[0] = move_up(rope[0]),
            "R" => rope[0] = move_right(rope[0]),
            "D" => rope[0] = move_down(rope[0]),
            "L" => rope[0] = move_left(rope[0]),
            x => return Err(anyhow!("Invalid direction: {}", x)),
        }

        (1..rope.len()).for_each(|index| {
            rope[index] = tug(rope[index - 1], rope[index]);
        });

        trail.insert(rope[rope.len() - 1]);

        Ok(())
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 13);

        Ok(())
    }

    #[test]
    fn example_2a() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 1);

        Ok(())
    }

    #[test]
    fn example_2b() -> Result<()> {
        let input = r"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

        assert_eq!(part_2(input.trim().to_string())?, 36);

        Ok(())
    }
}
