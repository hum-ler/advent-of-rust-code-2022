use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-8.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let grid = convert_input_into_grid(input);

    count_visible_trees(&grid)
}

fn part_2(input: String) -> Result<usize> {
    let grid = convert_input_into_grid(input);

    max_scenic_score(&grid)
}

fn convert_input_into_grid(input: String) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|byte| byte - b'0').collect())
        .collect()
}

/// Transposes the square grid from rows into columns, and vice versa.
fn transpose(grid: &[Vec<u8>]) -> Result<Vec<Vec<u8>>> {
    let grid_size = grid.len();

    let mut transpose: Vec<Vec<u8>> = Vec::new();
    grid.iter().try_for_each(|row| {
        if row.len() != grid_size {
            return Err(anyhow!("Grid is not a square"));
        }

        transpose.push(vec![0; grid_size]);

        Ok(())
    })?;

    for (row, bytes) in grid.iter().enumerate() {
        for (col, byte) in bytes.iter().enumerate() {
            transpose[col][row] = *byte;
        }
    }

    Ok(transpose)
}

fn count_visible_trees(grid_by_rows: &[Vec<u8>]) -> Result<usize> {
    let grid_size = grid_by_rows.len();

    let grid_by_cols = transpose(grid_by_rows)?;

    let mut visible_count = 0;
    for (row, trees) in grid_by_rows.iter().enumerate() {
        for (col, tree) in trees.iter().enumerate() {
            if row == 0 || row == grid_size - 1 || col == 0 || col == grid_size - 1 {
                visible_count += 1;
                continue;
            }

            if is_visible(
                *tree,
                &grid_by_cols[col][..row],
                &grid_by_rows[row][(col + 1)..],
                &grid_by_cols[col][(row + 1)..],
                &grid_by_rows[row][..col],
            ) {
                visible_count += 1;
            }
        }
    }

    Ok(visible_count)
}

fn is_visible(height: u8, n: &[u8], e: &[u8], s: &[u8], w: &[u8]) -> bool {
    n.iter().all(|tree| *tree < height)
        || e.iter().all(|tree| *tree < height)
        || s.iter().all(|tree| *tree < height)
        || w.iter().all(|tree| *tree < height)
}

fn max_scenic_score(grid_by_rows: &[Vec<u8>]) -> Result<usize> {
    let grid_by_cols = transpose(grid_by_rows)?;

    let mut max_scenic_score = 0;
    for (row, trees) in grid_by_rows.iter().enumerate() {
        for (col, tree) in trees.iter().enumerate() {
            // Including visible trees.

            let scenic_score = scenic_score(
                *tree,
                &grid_by_cols[col][..row],
                &grid_by_rows[row][(col + 1)..],
                &grid_by_cols[col][(row + 1)..],
                &grid_by_rows[row][..col],
            );
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    Ok(max_scenic_score)
}

fn scenic_score(height: u8, n: &[u8], e: &[u8], s: &[u8], w: &[u8]) -> usize {
    direction_scenic_score(height, n.iter().rev())
        * direction_scenic_score(height, e.iter())
        * direction_scenic_score(height, s.iter())
        * direction_scenic_score(height, w.iter().rev())
}

/// Counts the number of trees before view is blocked (count includes the blocking tree).
///
/// Note the puzzle is worded ambiguously -- your view of a shorter tree behind a taller tree is not
/// blocked -- just so long as the taller tree does not block your view entirely.
fn direction_scenic_score<'a, T>(height: u8, trees_iter: T) -> usize
where
    T: Iterator<Item = &'a u8>,
{
    trees_iter
        .fold((0usize, true), |acc, tree| {
            let (count, to_continue) = acc;

            if !to_continue {
                return acc;
            }

            if *tree >= height {
                (count + 1, false)
            } else {
                (count + 1, true)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
30373
25512
65332
33549
35390
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 21);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 8);

        Ok(())
    }
}
