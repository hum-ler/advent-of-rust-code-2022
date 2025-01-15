use std::ops::RangeInclusive;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-15.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    blankout_positions_at_y(input, 2000000)
}

fn part_2(input: String) -> Result<isize> {
    find_tuning_frequency(input, 0, 4000000)
}

type Coord = (isize, isize);

/// Represents a sensor.
struct Sensor {
    /// The location of the sensor.
    pos: Coord,

    /// The distance to the detected beacon.
    range: usize,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Beacon {
    pos: Coord,
}

impl Sensor {
    /// Checks whether pos is in the range of this sensor.
    fn is_in_range(&self, pos: Coord) -> bool {
        manhattan_distance(self.pos, pos) <= self.range
    }

    /// Checks whether the range of this sensor reaches y_index.
    fn is_in_range_at_y(&self, y_index: isize) -> bool {
        self.is_in_range((self.pos.0, y_index))
    }

    /// Finds the range of x values covered by this sensor at y_index.
    fn coverage_range_at_y(&self, y_index: isize) -> Option<RangeInclusive<isize>> {
        if !self.is_in_range_at_y(y_index) {
            return None;
        }

        let distance = self.pos.1.abs_diff(y_index);

        Some(
            (self.pos.0 - (self.range - distance) as isize)
                ..=(self.pos.0 + (self.range - distance) as isize),
        )
    }

    /// Finds the y indices and ranges of x values covered by this sensor.
    fn coverage_ranges(&self) -> Vec<(isize, RangeInclusive<isize>)> {
        let mut ranges = vec![(
            self.pos.1,
            ((self.pos.0 - self.range as isize)..=(self.pos.0 + self.range as isize)),
        )];

        (1..=self.range).for_each(|distance| {
            ranges.push((
                (self.pos.1 + distance as isize),
                (self.pos.0 - (self.range - distance) as isize)
                    ..=(self.pos.0 + (self.range - distance) as isize),
            ));
            ranges.push((
                (self.pos.1 - distance as isize),
                (self.pos.0 - (self.range - distance) as isize)
                    ..=(self.pos.0 + (self.range - distance) as isize),
            ));
        });

        ranges
    }
}

/// Finds the number of positions that cannot contain a beacon along y_index.
fn blankout_positions_at_y(input: String, y_index: isize) -> Result<usize> {
    let (sensors, beacons) = parse_input_into_sensors_beacons(input)?;

    let sensor_count_at_y = sensors
        .iter()
        .filter(|sensor| sensor.pos.1 == y_index)
        .count();

    let beacon_count_at_y = beacons
        .iter()
        .filter(|beacon| beacon.pos.1 == y_index)
        .count();

    let mut coverage_range_at_y: Vec<RangeInclusive<isize>> = Vec::new();
    for sensor in sensors {
        if let Some(range) = sensor.coverage_range_at_y(y_index) {
            coverage_range_at_y = range.insert_into(&coverage_range_at_y);
        }
    }
    let coverage_at_y = coverage_range_at_y
        .into_iter()
        .map(|range| range.count())
        .sum::<usize>();

    Ok(coverage_at_y - sensor_count_at_y - beacon_count_at_y)
}

/// Find the tuning frequency of the distress beacon.
fn find_tuning_frequency(input: String, min_index: isize, max_index: isize) -> Result<isize> {
    let (sensors, _) = parse_input_into_sensors_beacons(input)?;

    // Initialize a grid that is the size of the window of interest. Each element in the vec is the
    // range that is not covered by a sensor.
    let mut grid: Vec<Vec<RangeInclusive<isize>>> = Vec::new();
    for _ in min_index..=max_index {
        grid.push(vec![min_index..=max_index]);
    }

    sensors.iter().for_each(|sensor| {
        sensor.coverage_ranges().iter().for_each(|(y, range)| {
            // Check that we are in the window of interest.
            if *y < min_index || *y > max_index {
                return;
            }

            let row = &grid[*y as usize].clone();

            // Check that we still have something to substract from.
            if row.is_empty() {
                return;
            }

            // Update the grid with the updated range.
            let row = row
                .iter()
                .flat_map(|prev_range| prev_range.subtract(range))
                .collect::<Vec<_>>();
            grid[*y as usize] = row;
        });
    });

    // Locate the only row that still have the single pos left.
    let distress_beacon = grid
        .into_iter()
        .enumerate()
        .filter_map(|(y, row)| {
            if !row.is_empty() {
                Some((*row[0].start(), y as isize))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let Some((x, y)) = distress_beacon.first() else {
        return Err(anyhow!("Cannot locate distress beacon"));
    };

    Ok(*x * 4000000 + *y)
}

/// Extends some functionality to ranges.
trait RangeExtension
where
    Self: Sized,
{
    /// Adds the other range to this range.
    fn add(&self, other: &Self) -> Vec<Self>;

    /// Subtracts the other range from this range.
    fn subtract(&self, other: &Self) -> Vec<Self>;

    /// Checks whether this range is a superset of the other range.
    fn is_superset_of(&self, other: &Self) -> bool;

    /// Checks whether this range is a subset of the other range.
    fn is_subset_of(&self, other: &Self) -> bool {
        other.is_superset_of(self)
    }

    /// Checks whether this range is to the left of- and not intersecting the other range.
    fn is_left_of(&self, other: &Self) -> bool;

    /// Checks whether this range is to the right of- and not intersecting the other range.
    fn is_right_of(&self, other: &Self) -> bool;

    /// Checks whether this range is not intersecting the other range.
    fn is_disjoint_to(&self, other: &Self) -> bool {
        self.is_left_of(other) || self.is_right_of(other)
    }

    /// Checks whether this range intersects the other range from the left.
    fn intersects_start_of(&self, other: &Self) -> bool;

    /// Checks whether this range intersects the other range from the right.
    fn intersects_end_of(&self, other: &Self) -> bool;

    /// Inserts this range into a list of ranges sorted from left to right.
    ///
    /// Combines all ranges that are not disjoint.
    fn insert_into(&self, ranges: &[Self]) -> Vec<Self>;
}

impl RangeExtension for RangeInclusive<isize> {
    fn add(&self, other: &Self) -> Vec<Self> {
        match (self, other) {
            (this, other) if this.is_superset_of(other) => vec![this.clone()],
            (this, other) if this.is_subset_of(other) => vec![other.clone()],
            (this, other) if this.is_left_of(other) => vec![this.clone(), other.clone()],
            (this, other) if this.is_right_of(other) => vec![other.clone(), this.clone()],
            (this, other) if this.intersects_start_of(other) => vec![*this.start()..=*other.end()],
            (this, other) if this.intersects_end_of(other) => vec![*other.start()..=*this.end()],
            _ => unreachable!(),
        }
    }

    fn subtract(&self, other: &Self) -> Vec<Self> {
        match (self, other) {
            (this, other) if this.is_superset_of(other) => {
                let mut ranges = Vec::new();

                if this.start() != other.start() {
                    ranges.push(*this.start()..=(other.start() - 1));
                }
                if this.end() != other.end() {
                    ranges.push((*other.end() + 1)..=*this.end());
                }

                ranges
            }
            (this, other) if this.is_subset_of(other) => Vec::new(),
            (this, other) if this.is_disjoint_to(other) => vec![this.clone()],
            (this, other) if this.intersects_start_of(other) => {
                vec![*this.start()..=(*other.start() - 1)]
            }
            (this, other) if this.intersects_end_of(other) => {
                vec![(*other.end() + 1)..=*this.end()]
            }
            _ => unreachable!(),
        }
    }

    fn is_superset_of(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    fn is_left_of(&self, other: &Self) -> bool {
        self.end() < other.start()
    }

    fn is_right_of(&self, other: &Self) -> bool {
        self.start() > other.end()
    }

    fn intersects_start_of(&self, other: &Self) -> bool {
        self.end() >= other.start() && self.end() < other.end()
    }

    fn intersects_end_of(&self, other: &Self) -> bool {
        self.start() <= other.end() && self.start() > other.start()
    }

    fn insert_into(&self, ranges: &[Self]) -> Vec<Self> {
        if ranges.is_empty() {
            return vec![self.clone()];
        }

        let mut ranges = Vec::from(ranges);

        // Find a position to slot the range in.
        if let Some(index) = ranges.iter().position(|range| !self.is_right_of(range)) {
            ranges.insert(index, self.clone());
        } else {
            // No position, just insert at the very end and return.
            ranges.push(self.clone());
            return ranges;
        }

        // Reduce the ranges where possible.
        ranges.into_iter().fold(Vec::new(), |mut acc, range| {
            if acc.is_empty() {
                acc.push(range);
                return acc;
            }

            if let Some(prev_range) = acc.pop() {
                if prev_range.is_left_of(&range) {
                    acc.push(prev_range);
                    acc.push(range);
                } else {
                    let combined_range = prev_range.add(&range)[0].clone();
                    acc.push(combined_range);
                }
            }

            acc
        })
    }
}

fn parse_input_into_sensors_beacons(input: String) -> Result<(Vec<Sensor>, Vec<Beacon>)> {
    let regex = Regex::new(
        r"Sensor at x=(?<sx>-?\d+), y=(?<sy>-?\d+):.+is at x=(?<bx>-?\d+), y=(?<by>-?\d+)",
    )?;

    let pairs = input
        .lines()
        .map(|line| {
            let Some(captures) = regex.captures(line) else {
                return Err(anyhow!("Cannot parse regex on line: {}", line));
            };

            let sx = captures["sx"].parse()?;
            let sy = captures["sy"].parse()?;
            let bx = captures["bx"].parse()?;
            let by = captures["by"].parse()?;

            let range = manhattan_distance((sx, sy), (bx, by));

            Ok((
                Sensor {
                    pos: (sx, sy),
                    range,
                },
                Beacon { pos: (bx, by) },
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    let (sensors, beacons): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

    Ok((sensors, beacons.iter().unique().copied().collect()))
}

fn manhattan_distance(a: Coord, b: Coord) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(
            blankout_positions_at_y(EXAMPLE_INPUT.trim().to_string(), 10)?,
            26
        );

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(
            find_tuning_frequency(EXAMPLE_INPUT.trim().to_string(), 0, 20)?,
            56000011
        );

        Ok(())
    }

    #[test]
    fn range_subtraction() -> Result<()> {
        assert_eq!((2..=5).subtract(&(3..=4)), vec![(2..=2), (5..=5)]);
        assert_eq!((2..=5).subtract(&(2..=4)), vec![(5..=5)]);
        assert_eq!((2..=5).subtract(&(3..=5)), vec![(2..=2)]);

        assert_eq!((2..=5).subtract(&(1..=3)), vec![(4..=5)]);
        assert_eq!((2..=5).subtract(&(4..=6)), vec![(2..=3)]);

        assert_eq!((2..=5).subtract(&(1..=6)), Vec::new());
        assert_eq!((2..=5).subtract(&(2..=6)), Vec::new());
        assert_eq!((2..=5).subtract(&(1..=5)), Vec::new());

        assert_eq!((2..=5).subtract(&(2..=5)), Vec::new());

        Ok(())
    }

    #[test]
    fn range_addition() -> Result<()> {
        assert_eq!((2..=5).add(&(3..=4)), vec![(2..=5)]);
        assert_eq!((2..=5).add(&(2..=4)), vec![(2..=5)]);
        assert_eq!((2..=5).add(&(3..=5)), vec![(2..=5)]);

        assert_eq!((2..=5).add(&(1..=3)), vec![(1..=5)]);
        assert_eq!((2..=5).add(&(4..=6)), vec![(2..=6)]);

        assert_eq!((2..=5).add(&(1..=6)), vec![(1..=6)]);
        assert_eq!((2..=5).add(&(2..=6)), vec![(2..=6)]);
        assert_eq!((2..=5).add(&(1..=5)), vec![(1..=5)]);

        assert_eq!((2..=5).add(&(2..=5)), vec![(2..=5)]);

        Ok(())
    }
}
