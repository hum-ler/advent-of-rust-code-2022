use anyhow::{Result, anyhow};
use pathfinding::prelude::dijkstra;

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-12.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: String) -> Result<usize> {
    let (grid, grid_size, start, end) = convert_input_into_grid(input)?;

    let Some((_, length)) = dijkstra(
        &start,
        |node| successors(node, &grid, &grid_size),
        |node| *node == end,
    ) else {
        return Err(anyhow!("Cannot find shortest path"));
    };

    Ok(length)
}

fn part_2(input: String) -> Result<usize> {
    let (grid, grid_size, _, end) = convert_input_into_grid(input)?;

    let starts = grid
        .iter()
        .enumerate()
        .flat_map(|(row, bytes)| {
            bytes
                .iter()
                .enumerate()
                .filter_map(|(col, byte)| {
                    if *byte == b'a' {
                        Some((row, col))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    starts
        .iter()
        .filter_map(|start| {
            if let Some((_, length)) = dijkstra(
                start,
                |node| successors(node, &grid, &grid_size),
                |node| *node == end,
            ) {
                Some(length)
            } else {
                None
            }
        })
        .min()
        .ok_or(anyhow!("Cannot find min length"))
}

type Coord = (usize, usize);

type GridSize = (usize, usize);

/// Converts the input into the grid, and the start and end [Coord]s.
fn convert_input_into_grid(input: String) -> Result<(Vec<Vec<u8>>, GridSize, Coord, Coord)> {
    let mut grid = input
        .lines()
        .map(|line| line.bytes().collect())
        .collect::<Vec<Vec<_>>>();

    let grid_size = (grid.len(), grid.first().map_or(0, Vec::len));

    let Some(start_row) = grid.iter().position(|row| row.contains(&b'S')) else {
        return Err(anyhow!("Cannot find start row"));
    };
    let Some(start_col) = grid[start_row].iter().position(|byte| *byte == b'S') else {
        return Err(anyhow!("Cannot find start col"));
    };
    grid[start_row][start_col] = b'a';

    let Some(end_row) = grid.iter().position(|row| row.contains(&b'E')) else {
        return Err(anyhow!("Cannot find end row"));
    };
    let Some(end_col) = grid[end_row].iter().position(|byte| *byte == b'E') else {
        return Err(anyhow!("Cannot find end col"));
    };
    grid[end_row][end_col] = b'z';

    Ok((grid, grid_size, (start_row, start_col), (end_row, end_col)))
}

/// Finds [Coord]s that are reachable from node (1 level higher, same or any level below).
fn successors(node: &Coord, grid: &[Vec<u8>], grid_size: &GridSize) -> Vec<(Coord, usize)> {
    let &(row, col) = node;
    let &(row_count, col_count) = grid_size;
    let elevation = grid[row][col];

    let mut nodes = Vec::new();

    // n
    if row > 0 && grid[row - 1][col] <= elevation + 1 {
        nodes.push(((row - 1, col), 1));
    }

    // e
    if col < col_count - 1 && grid[row][col + 1] <= elevation + 1 {
        nodes.push(((row, col + 1), 1));
    }

    // s
    if row < row_count - 1 && grid[row + 1][col] <= elevation + 1 {
        nodes.push(((row + 1, col), 1));
    }

    // w
    if col > 0 && grid[row][col - 1] <= elevation + 1 {
        nodes.push(((row, col - 1), 1));
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 31);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 29);

        Ok(())
    }
}
