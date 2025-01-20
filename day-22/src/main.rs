use std::collections::HashSet;

use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-22.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let mut board = Board::try_from(input)?;

    board.run()?;

    Ok((board.row + 1) * 1000 + (board.col + 1) * 4 + board.facing)
}

fn part_2(input: String) -> Result<usize> {
    let mut cube = Cube::try_from(input)?;

    cube.run()?;

    let (row, col) = cube.abs_pos()?;

    Ok((row + 1) * 1000 + (col + 1) * 4 + cube.abs_facing())
}

#[derive(Clone)]
enum Movement {
    MoveForward(usize),
    TurnLeft,
    TurnRight,
}

struct Board {
    row: usize,
    col: usize,
    facing: usize,
    row_bounds: Vec<(usize, usize)>,
    col_bounds: Vec<(usize, usize)>,
    obstacles: HashSet<(usize, usize)>,
    movements: Vec<Movement>,
}

impl TryFrom<String> for Board {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        let Some((grid_part, movements_part)) = value.split_once("\n\n") else {
            return Err(anyhow!("Cannot split value: {}", value));
        };

        let lines = grid_part.lines().collect::<Vec<_>>();
        let col_count = lines
            .iter()
            .max_by_key(|line| line.len())
            .ok_or(anyhow!("Cannot find max col"))?
            .len();

        let mut row_bounds = Vec::new();
        let mut col_bounds = vec![(usize::MAX, usize::MIN); col_count];
        let mut obstacles = HashSet::new();

        for (row, line) in lines.iter().enumerate() {
            // Find row bound.
            let Some(left_bound) = line.find(|c: char| c.is_ascii_punctuation()) else {
                return Err(anyhow!("Cannot find left bound for line: {}", line));
            };
            let right_bound = line.len() - 1;
            row_bounds.push((left_bound, right_bound));

            // Find col bound.
            (left_bound..=right_bound).for_each(|col| {
                if row < col_bounds[col].0 {
                    col_bounds[col].0 = row;
                }
                if row > col_bounds[col].1 {
                    col_bounds[col].1 = row;
                }
            });

            // Find obstacles.
            line.match_indices("#").for_each(|(col, _)| {
                obstacles.insert((row, col));
            });
        }

        let row = 0;
        let Some(col) = lines[row].find(".") else {
            return Err(anyhow!("Cannot find col of starting pos"));
        };

        let movements = parse_movements(movements_part)?;

        Ok(Board {
            row,
            col,
            facing: 0,
            row_bounds,
            col_bounds,
            obstacles,
            movements,
        })
    }
}

impl Board {
    fn move_forward(&mut self, steps: usize) -> Result<()> {
        match self.facing {
            // n
            3 => {
                for _ in 0..steps {
                    let next_row = if self.row == self.col_bounds[self.col].0 {
                        self.col_bounds[self.col].1
                    } else {
                        self.row - 1
                    };

                    if self.obstacles.contains(&(next_row, self.col)) {
                        break;
                    } else {
                        self.row = next_row;
                    }
                }
            }

            // e
            0 => {
                for _ in 0..steps {
                    let next_col = if self.col == self.row_bounds[self.row].1 {
                        self.row_bounds[self.row].0
                    } else {
                        self.col + 1
                    };

                    if self.obstacles.contains(&(self.row, next_col)) {
                        break;
                    } else {
                        self.col = next_col;
                    }
                }
            }

            // s
            1 => {
                for _ in 0..steps {
                    let next_row = if self.row == self.col_bounds[self.col].1 {
                        self.col_bounds[self.col].0
                    } else {
                        self.row + 1
                    };

                    if self.obstacles.contains(&(next_row, self.col)) {
                        break;
                    } else {
                        self.row = next_row;
                    }
                }
            }

            // w
            2 => {
                for _ in 0..steps {
                    let next_col = if self.col == self.row_bounds[self.row].0 {
                        self.row_bounds[self.row].1
                    } else {
                        self.col - 1
                    };

                    if self.obstacles.contains(&(self.row, next_col)) {
                        break;
                    } else {
                        self.col = next_col;
                    }
                }
            }
            x => return Err(anyhow!("Invalid facing: {}", x)),
        }

        Ok(())
    }

    fn turn_left(&mut self) {
        self.facing = self.facing.wrapping_sub(1) % 4;
    }

    fn turn_right(&mut self) {
        self.facing = (self.facing + 1) % 4;
    }

    fn run(&mut self) -> Result<()> {
        for movement in self.movements.clone() {
            match movement {
                Movement::MoveForward(steps) => self.move_forward(steps)?,
                Movement::TurnLeft => self.turn_left(),
                Movement::TurnRight => self.turn_right(),
            }
        }

        Ok(())
    }
}

fn parse_movements(input: &str) -> Result<Vec<Movement>> {
    let mut movements = Vec::new();

    input.as_bytes().iter().try_for_each(|byte| match byte {
        b'L' => {
            movements.push(Movement::TurnLeft);

            Ok(())
        }
        b'R' => {
            movements.push(Movement::TurnRight);

            Ok(())
        }
        x if x.is_ascii_digit() => {
            if let Some(Movement::MoveForward(_)) = movements.last() {
                let Some(Movement::MoveForward(prev_steps)) = movements.pop() else {
                    return Err(anyhow!("Cannot match popped Movement to MoveForward"));
                };
                movements.push(Movement::MoveForward(prev_steps * 10 + (x - b'0') as usize));
            } else {
                movements.push(Movement::MoveForward((x - b'0') as usize));
            }

            Ok(())
        }
        x => Err(anyhow!("Unhandled movement: {}", x)),
    })?;

    Ok(movements)
}

// (square_index, row, col, facing)
type Position = (usize, usize, usize, usize);

type Transition = Box<dyn Fn(Position) -> Position>;

type Square = [Transition; 4];

struct Cube {
    position: Position,
    square_size: usize,
    squares: [Square; 6],
    obstacles: HashSet<(usize, usize, usize)>,
    movements: Vec<Movement>,
}

impl TryFrom<String> for Cube {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        let Some((squares_part, movements_part)) = value.split_once("\n\n") else {
            return Err(anyhow!("Cannot split value: {}", value));
        };

        let lines = squares_part.lines().collect::<Vec<_>>();
        let first_line_len = lines.first().map_or(0, |line| line.len());
        if first_line_len == 0 {
            return Err(anyhow!("Invalid input"));
        }

        let position = (0, 0, 0, 0);
        let square_size = if first_line_len % 50 == 0 { 50 } else { 4 };
        let squares = Self::generate_squares(square_size)?;

        let mut obstacles = HashSet::new();
        for (row, line) in squares_part.lines().enumerate() {
            line.match_indices("#").try_for_each(|(col, _)| {
                obstacles.insert(Self::abs_pos_to_rel_pos(square_size, row, col)?);

                Ok::<_, anyhow::Error>(())
            })?;
        }

        let movements = parse_movements(movements_part)?;

        Ok(Cube {
            position,
            square_size,
            squares,
            obstacles,
            movements,
        })
    }
}

impl Cube {
    fn run(&mut self) -> Result<()> {
        for movement in self.movements.clone() {
            match movement {
                Movement::MoveForward(steps) => self.move_forward(steps)?,
                Movement::TurnLeft => self.turn_left(),
                Movement::TurnRight => self.turn_right(),
            }
        }

        Ok(())
    }

    fn move_forward(&mut self, steps: usize) -> Result<()> {
        let lower_bound = 0;
        let upper_bound = self.square_size - 1;

        for _ in 0..steps {
            match self.position.3 {
                // n
                3 => {
                    let next_position = if self.position.1 == lower_bound {
                        self.squares[self.position.0][3](self.position)
                    } else {
                        (
                            self.position.0,
                            self.position.1 - 1,
                            self.position.2,
                            self.position.3,
                        )
                    };

                    if self
                        .obstacles
                        .contains(&(next_position.0, next_position.1, next_position.2))
                    {
                        break;
                    } else {
                        self.position = next_position;
                    }
                }

                // e
                0 => {
                    let next_position = if self.position.2 == upper_bound {
                        self.squares[self.position.0][0](self.position)
                    } else {
                        (
                            self.position.0,
                            self.position.1,
                            self.position.2 + 1,
                            self.position.3,
                        )
                    };

                    if self
                        .obstacles
                        .contains(&(next_position.0, next_position.1, next_position.2))
                    {
                        break;
                    } else {
                        self.position = next_position;
                    }
                }

                // s
                1 => {
                    let next_position = if self.position.1 == upper_bound {
                        self.squares[self.position.0][1](self.position)
                    } else {
                        (
                            self.position.0,
                            self.position.1 + 1,
                            self.position.2,
                            self.position.3,
                        )
                    };

                    if self
                        .obstacles
                        .contains(&(next_position.0, next_position.1, next_position.2))
                    {
                        break;
                    } else {
                        self.position = next_position;
                    }
                }

                // w
                2 => {
                    let next_position = if self.position.2 == lower_bound {
                        self.squares[self.position.0][2](self.position)
                    } else {
                        (
                            self.position.0,
                            self.position.1,
                            self.position.2 - 1,
                            self.position.3,
                        )
                    };

                    if self
                        .obstacles
                        .contains(&(next_position.0, next_position.1, next_position.2))
                    {
                        break;
                    } else {
                        self.position = next_position;
                    }
                }
                x => return Err(anyhow!("Invalid facing: {}", x)),
            }
        }

        Ok(())
    }

    fn turn_left(&mut self) {
        self.position.3 = self.position.3.wrapping_sub(1) % 4;
    }

    fn turn_right(&mut self) {
        self.position.3 = (self.position.3 + 1) % 4;
    }

    fn abs_pos(&self) -> Result<(usize, usize)> {
        Self::rel_pos_to_abs_pos(
            self.square_size,
            self.position.0,
            self.position.1,
            self.position.2,
        )
    }

    fn abs_facing(&self) -> usize {
        self.position.3
    }

    fn abs_pos_to_rel_pos(
        square_size: usize,
        row: usize,
        col: usize,
    ) -> Result<(usize, usize, usize)> {
        // Unfortunately, the example and input cubes have different shapes.

        match square_size {
            50 => Ok(match (row, col) {
                (row, col) if row < 50 && col < 100 => (0, row, col - 50),
                (row, col) if row < 50 => (1, row, col - 100),
                (row, col) if row < 100 => (2, row - 50, col - 50),
                (row, col) if row < 150 && col < 50 => (3, row - 100, col),
                (row, col) if row < 150 => (4, row - 100, col - 50),
                (row, col) => (5, row - 150, col),
            }),
            4 => Ok(match (row, col) {
                (row, col) if row < 4 => (0, row, col - 8),
                (row, col) if row < 8 && col < 4 => (1, row - 4, col),
                (row, col) if row < 8 && col < 8 => (2, row - 4, col - 4),
                (row, col) if row < 8 => (3, row - 4, col - 8),
                (row, col) if col < 12 => (4, row - 8, col - 8),
                (row, col) => (5, row - 8, col - 12),
            }),
            x => Err(anyhow!("Invalid square size: {}", x)),
        }
    }

    fn rel_pos_to_abs_pos(
        square_size: usize,
        square_index: usize,
        row: usize,
        col: usize,
    ) -> Result<(usize, usize)> {
        // Unfortunately, the example and input cubes have different shapes.

        match square_size {
            50 => Ok(match square_index {
                0 => (row, col + 50),
                1 => (row, col + 100),
                2 => (row + 50, col + 50),
                3 => (row + 100, col),
                4 => (row + 100, col + 50),
                5 => (row + 150, col),
                x => return Err(anyhow!("Invalid square index: {}", x)),
            }),
            4 => Ok(match square_index {
                0 => (row, col + 8),
                1 => (row + 4, col),
                2 => (row + 4, col + 4),
                3 => (row + 4, col + 8),
                4 => (row + 8, col + 8),
                5 => (row + 8, col + 12),
                x => return Err(anyhow!("Invalid square index: {}", x)),
            }),
            x => Err(anyhow!("Invalid square size: {}", x)),
        }
    }

    fn generate_squares(square_size: usize) -> Result<[Square; 6]> {
        // Unfortunately, the example and input cubes have different shapes.

        // Cube (50):  Square:   Bounds:
        //    [0][1]      3         L
        //    [2]      2 [ ] 0   L [ ] U
        // [3][4]         1         U
        // [5]
        //
        // Cube (4):
        //       [0]
        // [1][2][3]
        //       [4][5]

        let lower_bound = 0;
        let upper_bound = square_size - 1;

        match square_size {
            50 => {
                let square_0: Square = [
                    Box::new(move |pos| (1, pos.1, 0, 0)),
                    Box::new(move |pos| (2, lower_bound, pos.2, 1)),
                    Box::new(move |pos| (3, upper_bound - pos.1, lower_bound, 0)),
                    Box::new(move |pos| (5, pos.2, lower_bound, 0)),
                ];
                let square_1: Square = [
                    Box::new(move |pos| (4, upper_bound - pos.1, upper_bound, 2)),
                    Box::new(move |pos| (2, pos.2, upper_bound, 2)),
                    Box::new(move |pos| (0, pos.1, upper_bound, 2)),
                    Box::new(move |pos| (5, upper_bound, pos.2, 3)),
                ];
                let square_2: Square = [
                    Box::new(move |pos| (1, upper_bound, pos.1, 3)),
                    Box::new(move |pos| (4, lower_bound, pos.2, 1)),
                    Box::new(move |pos| (3, lower_bound, pos.1, 1)),
                    Box::new(move |pos| (0, upper_bound, pos.2, 3)),
                ];
                let square_3: Square = [
                    Box::new(move |pos| (4, pos.1, lower_bound, 0)),
                    Box::new(move |pos| (5, lower_bound, pos.2, 1)),
                    Box::new(move |pos| (0, upper_bound - pos.1, lower_bound, 0)),
                    Box::new(move |pos| (2, pos.2, lower_bound, 0)),
                ];
                let square_4: Square = [
                    Box::new(move |pos| (1, upper_bound - pos.1, upper_bound, 2)),
                    Box::new(move |pos| (5, pos.2, upper_bound, 2)),
                    Box::new(move |pos| (3, pos.1, upper_bound, 2)),
                    Box::new(move |pos| (2, upper_bound, pos.2, 3)),
                ];
                let square_5: Square = [
                    Box::new(move |pos| (4, upper_bound, pos.1, 3)),
                    Box::new(move |pos| (1, lower_bound, pos.2, 1)),
                    Box::new(move |pos| (0, lower_bound, pos.1, 1)),
                    Box::new(move |pos| (3, upper_bound, pos.2, 3)),
                ];

                Ok([square_0, square_1, square_2, square_3, square_4, square_5])
            }
            4 => {
                let square_0: Square = [
                    Box::new(move |pos| (5, upper_bound - pos.1, upper_bound, 2)),
                    Box::new(move |pos| (3, lower_bound, pos.2, 1)),
                    Box::new(move |pos| (2, lower_bound, pos.1, 1)),
                    Box::new(move |pos| (1, lower_bound, upper_bound - pos.2, 1)),
                ];
                let square_1: Square = [
                    Box::new(move |pos| (2, pos.1, lower_bound, 0)),
                    Box::new(move |pos| (4, upper_bound, upper_bound - pos.2, 3)),
                    Box::new(move |pos| (5, upper_bound, upper_bound - pos.1, 3)),
                    Box::new(move |pos| (0, lower_bound, upper_bound - pos.2, 1)),
                ];
                let square_2: Square = [
                    Box::new(move |pos| (3, pos.1, lower_bound, 0)),
                    Box::new(move |pos| (4, upper_bound - pos.2, lower_bound, 0)),
                    Box::new(move |pos| (1, pos.1, upper_bound, 2)),
                    Box::new(move |pos| (0, pos.2, lower_bound, 0)),
                ];
                let square_3: Square = [
                    Box::new(move |pos| (5, lower_bound, upper_bound - pos.1, 1)),
                    Box::new(move |pos| (4, lower_bound, pos.2, 1)),
                    Box::new(move |pos| (2, pos.1, upper_bound, 2)),
                    Box::new(move |pos| (0, upper_bound, pos.2, 3)),
                ];
                let square_4: Square = [
                    Box::new(move |pos| (5, pos.1, lower_bound, 0)),
                    Box::new(move |pos| (1, upper_bound, upper_bound - pos.2, 3)),
                    Box::new(move |pos| (2, upper_bound, upper_bound - pos.1, 3)),
                    Box::new(move |pos| (3, upper_bound, pos.2, 3)),
                ];
                let square_5: Square = [
                    Box::new(move |pos| (0, upper_bound - pos.1, upper_bound, 2)),
                    Box::new(move |pos| (1, upper_bound - pos.2, lower_bound, 0)),
                    Box::new(move |pos| (4, pos.1, upper_bound, 2)),
                    Box::new(move |pos| (3, upper_bound - pos.2, upper_bound, 2)),
                ];

                Ok([square_0, square_1, square_2, square_3, square_4, square_5])
            }
            x => Err(anyhow!("Invalid square size: {}", x)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(
            part_1(
                EXAMPLE_INPUT
                    .trim_start_matches("\n")
                    .trim_end_matches("\n")
                    .to_string()
            )?,
            6032
        );

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(
            part_2(
                EXAMPLE_INPUT
                    .trim_start_matches("\n")
                    .trim_end_matches("\n")
                    .to_string()
            )?,
            5031
        );

        Ok(())
    }
}
