use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use regex::Regex;

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-19.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let blueprints = input
        .lines()
        .map(Blueprint::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok(blueprints
        .into_par_iter()
        .map(|blueprint| {
            let mut statistic = vec![0; 25];
            let mut cache = HashMap::new();

            blueprint.id
                * max_geodes(
                    &blueprint,
                    24,
                    Inventory::default(),
                    &mut statistic,
                    &mut cache,
                )
        })
        .sum())
}

fn part_2(input: String) -> Result<usize> {
    let blueprints = input
        .lines()
        .map(Blueprint::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok(blueprints
        .into_par_iter()
        .take(3)
        .map(|blueprint| {
            let mut statistic = vec![0; 33];
            let mut cache = HashMap::new();

            max_geodes(
                &blueprint,
                32,
                Inventory::default(),
                &mut statistic,
                &mut cache,
            )
        })
        .product())
}

struct Blueprint {
    id: usize,
    ore_robot_ore_cost: usize,
    clay_robot_ore_cost: usize,
    obsidian_robot_ore_cost: usize,
    obsidian_robot_clay_cost: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obsidian_cost: usize,
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some(captures) = Regex::new(r"Blueprint (?<id>\d+):.+ore robot costs (?<oroc>\d+) ore.+clay robot costs (?<croc>\d+) ore.+obsidian robot costs (?<sroc>\d+) ore and (?<srcc>\d+) clay.+geode robot costs (?<groc>\d+) ore and (?<grsc>\d+) obsidian")?.captures(s) else {
            return Err(anyhow!("Cannot parse s: {}", s));
        };

        let id = captures["id"].parse()?;
        let ore_robot_ore_cost = captures["oroc"].parse()?;
        let clay_robot_ore_cost = captures["croc"].parse()?;
        let obsidian_robot_ore_cost = captures["sroc"].parse()?;
        let obsidian_robot_clay_cost = captures["srcc"].parse()?;
        let geode_robot_ore_cost = captures["groc"].parse()?;
        let geode_robot_obsidian_cost = captures["grsc"].parse()?;

        Ok(Blueprint {
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Inventory {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: usize,
    geode_robot: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            ore: Default::default(),
            clay: Default::default(),
            obsidian: Default::default(),
            geode: Default::default(),
            ore_robot: 1,
            clay_robot: Default::default(),
            obsidian_robot: Default::default(),
            geode_robot: Default::default(),
        }
    }
}

fn max_geodes(
    blueprint: &Blueprint,
    time: usize,
    inventory: Inventory,
    statistic: &mut Vec<usize>,
    cache: &mut HashMap<(usize, Inventory), usize>,
) -> usize {
    if time == 0 {
        if inventory.geode > statistic[0] {
            statistic[0] = inventory.geode;
        }

        return *cache.entry((time, inventory)).or_insert(inventory.geode);
    }

    if cache.contains_key(&(time, inventory)) {
        return cache[&(time, inventory)];
    }

    if inventory.geode + inventory.geode_robot + 2 < statistic[time - 1] {
        // It is a mystery why this heuristic work.
        //
        // Other unsuccessful attempts include:
        // (i) inventory.geode + time * (time - 1) / 2 * inventory.geode_robot < statistic[0]

        return *cache.entry((time, inventory)).or_insert(0);
    } else if inventory.geode > statistic[time] {
        statistic[time] = inventory.geode;
    }

    let mut options = Vec::new();

    // Add a geode robot.
    if inventory.ore >= blueprint.geode_robot_ore_cost
        && inventory.obsidian >= blueprint.geode_robot_obsidian_cost
    {
        options.push(Inventory {
            ore: inventory.ore - blueprint.geode_robot_ore_cost + inventory.ore_robot,
            clay: inventory.clay + inventory.clay_robot,
            obsidian: inventory.obsidian - blueprint.geode_robot_obsidian_cost
                + inventory.obsidian_robot,
            geode: inventory.geode + inventory.geode_robot,
            geode_robot: inventory.geode_robot + 1,
            ..inventory
        });
    }

    // Add an obsidian robot. We can stop adding obsidian robots once we reach
    // blueprint.geode_robot_obsidian_cost but this optimization doesn't seem to do much.
    if inventory.ore >= blueprint.obsidian_robot_ore_cost
        && inventory.clay >= blueprint.obsidian_robot_clay_cost
    {
        options.push(Inventory {
            ore: inventory.ore - blueprint.obsidian_robot_ore_cost + inventory.ore_robot,
            clay: inventory.clay - blueprint.obsidian_robot_clay_cost + inventory.clay_robot,
            obsidian: inventory.obsidian + inventory.obsidian_robot,
            geode: inventory.geode + inventory.geode_robot,
            obsidian_robot: inventory.obsidian_robot + 1,
            ..inventory
        });
    }

    // Add a clay robot. We can stop adding clay robots once we reach
    // blueprint.obsidian_robot_clay_cost but this optimization doesn't seem to do much.
    if inventory.ore >= blueprint.clay_robot_ore_cost {
        options.push(Inventory {
            ore: inventory.ore - blueprint.clay_robot_ore_cost + inventory.ore_robot,
            clay: inventory.clay + inventory.clay_robot,
            obsidian: inventory.obsidian + inventory.obsidian_robot,
            geode: inventory.geode + inventory.geode_robot,
            clay_robot: inventory.clay_robot + 1,
            ..inventory
        });
    }

    // Add an ore robot. We can stop adding ore robot once we have enough obsidian robots.
    if inventory.ore >= blueprint.ore_robot_ore_cost {
        options.push(Inventory {
            ore: inventory.ore - blueprint.ore_robot_ore_cost + inventory.ore_robot,
            clay: inventory.clay + inventory.clay_robot,
            obsidian: inventory.obsidian + inventory.obsidian_robot,
            geode: inventory.geode + inventory.geode_robot,
            ore_robot: inventory.ore_robot + 1,
            ..inventory
        });
    }

    // Add no robot.
    options.push(Inventory {
        ore: inventory.ore + inventory.ore_robot,
        clay: inventory.clay + inventory.clay_robot,
        obsidian: inventory.obsidian + inventory.obsidian_robot,
        geode: inventory.geode + inventory.geode_robot,
        ..inventory
    });

    let max = options
        .into_iter()
        .map(|inventory| max_geodes(blueprint, time - 1, inventory, statistic, cache))
        .max()
        .unwrap_or(0);

    *cache.entry((time, inventory)).or_insert(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 33);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 3472);

        Ok(())
    }
}
