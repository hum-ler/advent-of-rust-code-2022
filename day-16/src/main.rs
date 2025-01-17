use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use regex::Regex;

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-16.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<u16> {
    let valves = convert_input_to_valves(input)?;

    max_pressure_released(&valves)
}

fn part_2(input: String) -> Result<u16> {
    let valves = convert_input_to_valves(input)?;

    max_pressure_released_with_rerun(&valves)
}

#[allow(dead_code)]
fn example_2(input: String) -> Result<u16> {
    let valves = convert_input_to_valves(input)?;

    max_pressure_released_by_splitting(&valves)
}

struct Valve {
    id: u16,
    rate: u16,
    connections: Vec<u16>,
}

impl FromStr for Valve {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some(captures) = Regex::new(r"Valve (?<label>\w+) has flow rate=(?<rate>\d+); tunnels? leads? to valves? (?<connections>.+)")?.captures(s) else {
            return Err(anyhow!("Cannot parse s: {}", s));
        };

        let label = captures["label"].to_string();
        let id = label_to_id(&label);
        let rate = captures["rate"].parse()?;
        let connections = captures["connections"]
            .split_terminator(", ")
            .map(String::from)
            .map(|connection| label_to_id(&connection))
            .collect();

        Ok(Valve {
            id,
            rate,
            connections,
        })
    }
}

fn convert_input_to_valves(input: String) -> Result<HashMap<u16, Valve>> {
    input
        .lines()
        .map(Valve::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|valve| Ok((valve.id, valve)))
        .collect::<Result<HashMap<_, _>>>()
}

#[derive(Clone, Default)]
struct Trackables {
    time_left: u16,
    pressure_released: u16,
    flow_rate: u16,
    opened: HashSet<u16>,
    unopened: HashSet<u16>,
}

fn max_pressure_released(valves: &HashMap<u16, Valve>) -> Result<u16> {
    // We always regard valves with 0 rate as opened. Note that AA is 0 rate in both the example and
    // the input data.
    let trackables = Trackables {
        time_left: 30,
        pressure_released: 0,
        flow_rate: 0,
        opened: valves
            .values()
            .filter_map(|valve| {
                if valve.rate == 0 {
                    Some(valve.id)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>(),
        unopened: valves
            .values()
            .filter_map(|valve| {
                if valve.rate != 0 {
                    Some(valve.id)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>(),
    };

    let mut shortest_path_cache: HashMap<(u16, u16), Vec<u16>> = HashMap::new();

    find_max_pressure_released(
        label_to_id(&String::from("AA")),
        trackables,
        valves,
        &mut shortest_path_cache,
    )
    .map(|trackables| trackables.pressure_released)
}

fn max_pressure_released_with_rerun(valves: &HashMap<u16, Valve>) -> Result<u16> {
    // In 26 minutes, there will be a set of unopened valves left over in one optimized run. We can
    // simply re-run another 26 minutes to mop up those unopened valves. Unfortunately this strategy
    // does not work for the example, as the first run will open all valves within 26 minutes.

    // First run

    let trackables = Trackables {
        time_left: 26,
        pressure_released: 0,
        flow_rate: 0,
        opened: valves
            .values()
            .filter_map(|valve| {
                if valve.rate == 0 {
                    Some(valve.id)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>(),
        unopened: valves
            .values()
            .filter_map(|valve| {
                if valve.rate != 0 {
                    Some(valve.id)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>(),
    };

    let mut shortest_path_cache: HashMap<(u16, u16), Vec<u16>> = HashMap::new();

    let first_path = find_max_pressure_released(
        label_to_id(&String::from("AA")),
        trackables,
        valves,
        &mut shortest_path_cache,
    )?;

    // Second run

    let trackables = Trackables {
        time_left: 26,
        pressure_released: 0,
        flow_rate: 0,
        opened: first_path.opened.clone(),
        unopened: first_path.unopened.clone(),
    };

    let second_path = find_max_pressure_released(
        label_to_id(&String::from("AA")),
        trackables,
        valves,
        &mut shortest_path_cache,
    )?;

    Ok(first_path.pressure_released + second_path.pressure_released)
}

fn max_pressure_released_by_splitting(valves: &HashMap<u16, Valve>) -> Result<u16> {
    // The example should be small enough to brute force splitting the unopened valves into halves.

    let unopened = valves
        .values()
        .filter_map(|valve| {
            if valve.rate != 0 {
                Some(valve.id)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    unopened
        .iter()
        .combinations(unopened.len() / 2)
        .map(|first_half| {
            // Prepare the halves.
            let first_half = HashSet::from_iter(first_half.into_iter().copied());
            let second_half = unopened
                .difference(&first_half)
                .copied()
                .collect::<HashSet<_>>();

            // First half

            let trackables = Trackables {
                time_left: 26,
                pressure_released: 0,
                flow_rate: 0,
                opened: valves
                    .values()
                    .map(|valve| valve.id)
                    .collect::<HashSet<_>>()
                    .difference(&first_half)
                    .copied()
                    .collect::<HashSet<_>>(),
                unopened: first_half,
            };

            let mut shortest_path_cache: HashMap<(u16, u16), Vec<u16>> = HashMap::new();

            let first_half = find_max_pressure_released(
                label_to_id(&String::from("AA")),
                trackables,
                valves,
                &mut shortest_path_cache,
            )?;

            // Second half

            let trackables = Trackables {
                time_left: 26,
                pressure_released: 0,
                flow_rate: 0,
                opened: valves
                    .values()
                    .map(|valve| valve.id)
                    .collect::<HashSet<_>>()
                    .difference(&second_half)
                    .copied()
                    .collect::<HashSet<_>>(),
                unopened: second_half,
            };

            let second_half = find_max_pressure_released(
                label_to_id(&String::from("AA")),
                trackables,
                valves,
                &mut shortest_path_cache,
            )?;

            Ok(first_half.pressure_released + second_half.pressure_released)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .max()
        .ok_or(anyhow!("Cannot determine max from splits iters"))
}

fn find_max_pressure_released(
    start: u16,
    mut trackables: Trackables,
    valves: &HashMap<u16, Valve>,
    shortest_path_cache: &mut HashMap<(u16, u16), Vec<u16>>,
) -> Result<Trackables> {
    if trackables.time_left == 0 {
        return Ok(trackables);
    }

    if trackables.time_left == 1 {
        trackables.time_left = 0;
        trackables.pressure_released += trackables.flow_rate;
        return Ok(trackables);
    }

    if trackables.opened.len() == valves.len() {
        trackables.pressure_released += trackables.time_left * trackables.flow_rate;
        trackables.time_left = 0;
        return Ok(trackables);
    }

    trackables
        .unopened
        .iter()
        .map(|end| {
            let trackables =
                traverse(start, *end, trackables.clone(), valves, shortest_path_cache)?;

            find_max_pressure_released(*end, trackables, valves, shortest_path_cache)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .max_by_key(|trackables| trackables.pressure_released)
        .ok_or(anyhow!("Cannot determine max from trackables"))
}

/// Travels by shortest path from start valve to end valve (and opening it), tracking the stats.
fn traverse(
    start: u16,
    end: u16,
    mut trackables: Trackables,
    valves: &HashMap<u16, Valve>,
    shortest_path_cache: &mut HashMap<(u16, u16), Vec<u16>>,
) -> Result<Trackables> {
    if !shortest_path_cache.contains_key(&(start, end)) {
        let Some((shortest_path, _)) = dijkstra(
            &start,
            |valve_id| successors(valve_id, valves),
            |valve_id| *valve_id == end,
        ) else {
            return Err(anyhow!(
                "Cannot find path from {} to {}",
                id_to_label(start),
                id_to_label(end)
            ));
        };

        shortest_path_cache
            .entry((start, end))
            .or_insert(shortest_path.clone());
    }

    let shortest_path = shortest_path_cache[&(start, end)].clone();

    for _ in 1..shortest_path.len() {
        if trackables.time_left == 0 {
            return Ok(trackables);
        }

        trackables.pressure_released += trackables.flow_rate;
        trackables.time_left -= 1;
    }

    // Open the destination valve.

    if trackables.time_left == 0 {
        return Ok(trackables);
    }

    let end = shortest_path[shortest_path.len() - 1];

    trackables.pressure_released += trackables.flow_rate;
    trackables.time_left -= 1;

    trackables.flow_rate += valves[&end].rate;
    trackables.unopened.remove(&end);
    trackables.opened.insert(end);

    Ok(trackables)
}

/// Finds connections from valve id.
fn successors(id: &u16, valves: &HashMap<u16, Valve>) -> Vec<(u16, u16)> {
    valves[id]
        .connections
        .iter()
        .map(|connection_id| (*connection_id, 1))
        .collect()
}

/// Converts a 2-character string label to a number for use as ID.
fn label_to_id(label: &String) -> u16 {
    let bytes = label.as_bytes();

    ((bytes[0] as u16) << 8) + (bytes[1] as u16)
}

/// Converts an ID number back to the string label.
fn id_to_label(id: u16) -> String {
    let first_byte = (id >> 8) as u8;
    let second_byte = id as u8;

    String::from_utf8(vec![first_byte, second_byte]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 1651);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(super::example_2(EXAMPLE_INPUT.trim().to_string())?, 1707);

        Ok(())
    }
}
