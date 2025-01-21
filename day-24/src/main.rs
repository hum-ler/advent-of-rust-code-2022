use anyhow::{anyhow, Result};
use pathfinding::prelude::dijkstra;

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-24.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let grid = parse_input_into_grid(input)?;
    let rows = grid.len();
    let cols = grid[0].len();

    let mut safety_checks = Vec::new();
    for row in 0..rows {
        safety_checks.push(Vec::new());

        for col in 0..cols {
            safety_checks[row].push(tile_safety((row, col), rows, cols, &grid));
        }
    }

    // Start and end are outside of the grid.
    let start = 9998;
    let end = 9999;

    let Some((_, len)) = dijkstra(
        &(start, start, 0),
        |node| successors(node, rows, cols, &safety_checks, start, end),
        |node| node.0 == end && node.1 == end,
    ) else {
        return Err(anyhow!("Cannot find shortest path"));
    };

    Ok(len)
}

fn part_2(input: String) -> Result<usize> {
    let grid = parse_input_into_grid(input)?;
    let rows = grid.len();
    let cols = grid[0].len();

    let mut safety_checks = Vec::new();
    for row in 0..rows {
        safety_checks.push(Vec::new());

        for col in 0..cols {
            safety_checks[row].push(tile_safety((row, col), rows, cols, &grid));
        }
    }

    // Start and end are outside of the grid.
    let start = 9998;
    let end = 9999;

    let mut total_len = 0;

    let Some((_, len)) = dijkstra(
        &(start, start, total_len),
        |node| successors(node, rows, cols, &safety_checks, start, end),
        |node| node.0 == end && node.1 == end,
    ) else {
        return Err(anyhow!("Cannot find first shortest path to goal"));
    };

    total_len += len;

    let Some((_, len)) = dijkstra(
        &(end, end, total_len),
        |node| successors(node, rows, cols, &safety_checks, start, end),
        |node| node.0 == start && node.1 == start,
    ) else {
        return Err(anyhow!("Cannot find shortest path back to start"));
    };

    total_len += len;

    let Some((_, len)) = dijkstra(
        &(start, start, total_len),
        |node| successors(node, rows, cols, &safety_checks, start, end),
        |node| node.0 == end && node.1 == end,
    ) else {
        return Err(anyhow!("Cannot find second shortest path to goal"));
    };

    total_len += len;

    Ok(total_len)
}

type Coord = (usize, usize);

fn parse_input_into_grid(input: String) -> Result<Vec<Vec<u8>>> {
    let lines = input.lines().collect::<Vec<_>>();

    let line_count = lines.len();
    let line_len = lines.first().ok_or(anyhow!("Cannot get first line"))?.len();

    Ok(lines
        .into_iter()
        .skip(1)
        .take(line_count - 2)
        .map(|line| line.bytes().skip(1).take(line_len - 2).collect())
        .collect())
}

/// Gets a function to check the safety of the given tile at a specific step.
fn tile_safety(
    tile: Coord,
    rows: usize,
    cols: usize,
    grid: &[Vec<u8>],
) -> Box<dyn Fn(usize) -> bool> {
    // For each tile, we observe the blizzards that can occur within the same row or column:
    // - within the row, only '<' and '>' matter:
    //   - '>' position is (step + col) % cols.
    //   - '<' position is (cols + col - step % cols) % cols.
    // - within the column, only '^' and 'v' matter:
    //   - '^' position is (rows + row - step % rows) % rows.
    //   - 'v' position is (step + row) % rows.

    let (row, col) = tile;

    let right_blizzards = grid[row]
        .iter()
        .enumerate()
        .filter_map(|(col, byte)| if *byte == b'>' { Some(col) } else { None })
        .collect::<Vec<_>>();
    let left_blizzards = grid[row]
        .iter()
        .enumerate()
        .filter_map(|(col, byte)| if *byte == b'<' { Some(col) } else { None })
        .collect::<Vec<_>>();
    let up_blizzards = grid
        .iter()
        .enumerate()
        .filter_map(|(row, bytes)| if bytes[col] == b'^' { Some(row) } else { None })
        .collect::<Vec<_>>();
    let down_blizzards = grid
        .iter()
        .enumerate()
        .filter_map(|(row, bytes)| if bytes[col] == b'v' { Some(row) } else { None })
        .collect::<Vec<_>>();

    Box::new(move |step| {
        right_blizzards
            .iter()
            .all(|blizzard| col != (step + blizzard) % cols)
            && left_blizzards
                .iter()
                .all(|blizzard| col != (cols + blizzard - step % cols) % cols)
            && up_blizzards
                .iter()
                .all(|blizzard| row != (rows + blizzard - step % rows) % rows)
            && down_blizzards
                .iter()
                .all(|blizzard| row != (step + blizzard) % rows)
    })
}

type Node = (usize, usize, usize);

type SafetyChecks = Vec<Vec<Box<dyn Fn(usize) -> bool>>>;

fn successors(
    node: &Node,
    rows: usize,
    cols: usize,
    safety_checks: &SafetyChecks,
    start_index: usize,
    end_index: usize,
) -> Vec<(Node, usize)> {
    let &(row, col, step) = node;

    let mut nodes = Vec::new();

    // Start index is outside of the grid.
    if row == start_index && col == start_index {
        if safety_checks[0][0](step + 1) {
            nodes.push(((0, 0, step + 1), 1));
        }

        nodes.push(((start_index, start_index, step + 1), 1));

        return nodes;
    }

    // End index is outside of the grid.
    if row == end_index && col == end_index {
        if safety_checks[rows - 1][cols - 1](step + 1) {
            nodes.push(((rows - 1, cols - 1, step + 1), 1));
        }

        nodes.push(((end_index, end_index, step + 1), 1));

        return nodes;
    }

    // n
    if row > 0 && safety_checks[row - 1][col](step + 1) {
        nodes.push(((row - 1, col, step + 1), 1));
    }

    // e
    if col < cols - 1 && safety_checks[row][col + 1](step + 1) {
        nodes.push(((row, col + 1, step + 1), 1));
    }

    // s
    if row < rows - 1 && safety_checks[row + 1][col](step + 1) {
        nodes.push(((row + 1, col, step + 1), 1));
    }

    // w
    if col > 0 && safety_checks[row][col - 1](step + 1) {
        nodes.push(((row, col - 1, step + 1), 1));
    }

    // Wait at the same position.
    if safety_checks[row][col](step + 1) {
        nodes.push(((row, col, step + 1), 1));
    }

    // Start index is outside of the grid and always safe.
    if row == 0 && col == 0 {
        nodes.push(((start_index, start_index, step + 1), 1));
    }

    // End index is outside of the grid and always safe.
    if row == rows - 1 && col == cols - 1 {
        nodes.push(((end_index, end_index, step + 1), 1));
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 18);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 54);

        Ok(())
    }
}
