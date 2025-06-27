use std::{collections::HashMap, str::FromStr};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-21.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: String) -> Result<i64> {
    let mut monkeys = input
        .lines()
        .map(Monkey::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|monkey| (monkey.id, monkey))
        .collect::<HashMap<u32, Monkey>>();

    while resolve(&mut monkeys, &[])? > 0 {}

    let root = ascii_label_to_id("root")?;
    monkeys[&root]
        .number
        .ok_or(anyhow!("Cannot get root number"))
}

fn part_2(input: String) -> Result<i64> {
    let mut monkeys = input
        .lines()
        .map(Monkey::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|monkey| (monkey.id, monkey))
        .collect::<HashMap<u32, Monkey>>();

    let root = ascii_label_to_id("root")?;
    let humn = ascii_label_to_id("humn")?;

    monkeys
        .entry(humn)
        .and_modify(|monkey| monkey.number = None);

    while resolve(&mut monkeys, &[root, humn])? > 0 {}

    let root = monkeys[&root];
    let Some(input_1) = root.input_1 else {
        return Err(anyhow!("Cannot retrieve Input 1 from root"));
    };
    let Some(input_2) = root.input_2 else {
        return Err(anyhow!("Cannot retrieve Input 2 from root"));
    };

    if let Some(number_equates) = monkeys[&input_1].number {
        return solve_humn(input_2, number_equates, &monkeys);
    }

    if let Some(number_equates) = monkeys[&input_2].number {
        return solve_humn(input_1, number_equates, &monkeys);
    }

    Err(anyhow!("Both inputs to root are unresolved"))
}

#[derive(Clone, Copy, Debug)]
struct Monkey {
    id: u32,
    number: Option<i64>,
    input_1: Option<u32>,
    input_2: Option<u32>,
    operation: Option<fn(i64, i64) -> i64>,
    operator: Option<u8>,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((label, body)) = s.split_once(": ") else {
            return Err(anyhow!("Cannot split s: {}", s));
        };

        let id = ascii_label_to_id(label)?;
        let mut number = None;
        let mut input_1 = None;
        let mut input_2 = None;
        let mut operation = None;
        let mut operator = None;

        if let Ok(parsed_number) = body.parse() {
            number = Some(parsed_number);

            return Ok(Monkey {
                id,
                number,
                input_1,
                input_2,
                operation,
                operator,
            });
        }

        let split_body = body.split_terminator(" ").collect::<Vec<_>>();
        if !split_body.len() == 3 {
            return Err(anyhow!("Cannot split body: {}", body));
        }

        input_1 = ascii_label_to_id(split_body[0]).ok();
        input_2 = ascii_label_to_id(split_body[2]).ok();

        operation = Some(match split_body[1] {
            "+" => |x, y| x + y,
            "-" => |x, y| x - y,
            "*" => |x, y| x * y,
            "/" => |x, y| x / y, // all divisions are clean with no remainder
            _ => unreachable!(),
        });

        operator = Some(split_body[1].as_bytes()[0]);

        Ok(Monkey {
            id,
            number,
            input_1,
            input_2,
            operation,
            operator,
        })
    }
}

/// Iterates through monkeys and resolve numbers where possible. Use skip_ids to avoid updating
/// specific [Monkey]s.
///
/// Returns the number of [Monkey]s updated in this iteration.
fn resolve(monkeys: &mut HashMap<u32, Monkey>, skip_ids: &[u32]) -> Result<usize> {
    let mut updated_monkeys = 0usize;

    for monkey in monkeys.clone().values() {
        if skip_ids.contains(&monkey.id) {
            continue;
        }

        if monkey.number.is_some() {
            continue;
        }

        let Some(input_1) = monkey.input_1 else {
            return Err(anyhow!("Cannot get input 1 for Monkey {}", monkey.id));
        };
        let Some(number_1) = monkeys[&input_1].number else {
            continue;
        };

        let Some(input_2) = monkey.input_2 else {
            return Err(anyhow!("Cannot get input 2 for Monkey {}", monkey.id));
        };
        let Some(number_2) = monkeys[&input_2].number else {
            continue;
        };

        let Some(operation) = monkey.operation else {
            return Err(anyhow!("Cannot get operation for Monkey {}", monkey.id));
        };

        monkeys.entry(monkey.id).and_modify(|monkey| {
            monkey.number = Some(operation(number_1, number_2));
        });

        updated_monkeys += 1;
    }

    Ok(updated_monkeys)
}

/// Solves for humn recursively. id refers to an input to an ancestor [Monkey], while number_equates
/// is the other input that has already been resolved.
fn solve_humn(id: u32, number_equates: i64, monkeys: &HashMap<u32, Monkey>) -> Result<i64> {
    let humn = ascii_label_to_id("humn")?;

    let monkey = monkeys[&id];
    let Some(input_1) = monkey.input_1 else {
        return Err(anyhow!("Cannot retrieve input 1 from Monkey {}", monkey.id));
    };
    let input_1 = monkeys[&input_1];

    let Some(input_2) = monkey.input_2 else {
        return Err(anyhow!("Cannot retrieve input 2 from Monkey {}", monkey.id));
    };
    let input_2 = monkeys[&input_2];

    // Check base case.
    // humn is lhs for both the example and actual input.
    // humn <op> <input_2> == number_equates
    if input_1.id == humn {
        let Some(number) = input_2.number else {
            return Err(anyhow!("Cannot retrieve number from Monkey {}", input_2.id));
        };

        return match monkey.operator {
            Some(b'+') => Ok(number_equates - number),
            Some(b'-') => Ok(number_equates + number),
            Some(b'*') => Ok(number_equates / number),
            Some(b'/') => Ok(number_equates * number),
            _ => unreachable!(),
        };
    }

    // number <operator> <input_2> == number_equates
    if input_1.number.is_some() {
        let Some(number) = input_1.number else {
            return Err(anyhow!("Cannot retrieve number from Monkey {}", input_1.id));
        };

        return match monkey.operator {
            Some(b'+') => solve_humn(input_2.id, number_equates - number, monkeys),
            Some(b'-') => solve_humn(input_2.id, number - number_equates, monkeys),
            Some(b'*') => solve_humn(input_2.id, number_equates / number, monkeys),
            Some(b'/') => solve_humn(input_2.id, number / number_equates, monkeys),
            _ => unreachable!(),
        };
    }

    // <input_1> <operator> number == number_equates
    if input_2.number.is_some() {
        let Some(number) = input_2.number else {
            return Err(anyhow!("Cannot retrieve number from Monkey {}", input_2.id));
        };

        return match monkey.operator {
            Some(b'+') => solve_humn(input_1.id, number_equates - number, monkeys),
            Some(b'-') => solve_humn(input_1.id, number_equates + number, monkeys),
            Some(b'*') => solve_humn(input_1.id, number_equates / number, monkeys),
            Some(b'/') => solve_humn(input_1.id, number_equates * number, monkeys),
            _ => unreachable!(),
        };
    }

    Err(anyhow!("Invalid Monkey when solving humn: {}", monkey.id))
}

fn ascii_label_to_id(label: &str) -> Result<u32> {
    Ok(u32::from_be_bytes(label.as_bytes().try_into()?))
}

#[allow(dead_code)]
fn id_to_ascii_label(id: u32) -> Result<String> {
    String::from_utf8(id.to_be_bytes().to_vec())
        .map_err(|error| anyhow!("Cannot convert id to label: {}", error))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 152);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 301);

        Ok(())
    }
}
