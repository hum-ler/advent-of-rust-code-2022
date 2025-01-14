use std::collections::VecDeque;

use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-11.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let mut monkeys = parse_input_into_monkeys(input)?;

    (0..20).for_each(|_| round(&mut monkeys, None));

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    Ok(monkeys
        .into_iter()
        .take(2)
        .map(|monkey| monkey.inspection_count)
        .product())
}

fn part_2(input: String) -> Result<usize> {
    let mut monkeys = parse_input_into_monkeys(input)?;

    // Get the resultant modulus from divisors. All divisors must be pairwise coprime.
    let modulus = monkeys.iter().map(|monkey| monkey.divisor).product();

    (0..10000).for_each(|_| round(&mut monkeys, Some(modulus)));

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    Ok(monkeys
        .into_iter()
        .take(2)
        .map(|monkey| monkey.inspection_count)
        .product())
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> bool>,
    divisor: u64,
    true_target: usize,
    false_target: usize,
    inspection_count: usize,
}

fn parse_input_into_monkeys(input: String) -> Result<Vec<Monkey>> {
    input.split_terminator("\n\n").map(parse_monkey).collect()
}

fn parse_monkey(input: &str) -> Result<Monkey> {
    let lines = input.lines().collect::<Vec<_>>();
    if lines.len() != 6 {
        return Err(anyhow!("Unexpected input: {}", input));
    }
    if !lines[0].starts_with("Monkey") {
        return Err(anyhow!("Unexpected Monkey line: {}", lines[0]));
    }

    // items

    if !lines[1].contains("Starting items") {
        return Err(anyhow!("Unexpected Starting items line: {}", lines[1]));
    }
    let Some((_, items)) = lines[1].split_once(": ") else {
        return Err(anyhow!("Cannot split Starting items line: {}", lines[1]));
    };
    let items = items
        .split_terminator(", ")
        .map(str::parse::<u64>)
        .collect::<Result<VecDeque<_>, _>>()?;

    // operation

    if !lines[2].contains("Operation") {
        return Err(anyhow!("Unexpected Operation line: {}", lines[2]));
    }
    let Some((_, operation)) = lines[2].split_once("new = ") else {
        return Err(anyhow!("Cannot split Operation line: {}", lines[2]));
    };
    let operation: Box<dyn Fn(u64) -> u64> = match operation {
        "old + old" => Box::new(|item| item + item),
        "old * old" => Box::new(|item| item * item),
        add if add.starts_with("old +") => {
            let Some((_, operand)) = operation.split_once(" + ") else {
                return Err(anyhow!("Cannot split expression: {}", add));
            };

            let operand = operand.parse::<u64>()?;

            Box::new(move |item| item + operand)
        }
        multiply if multiply.starts_with("old *") => {
            let Some((_, operand)) = operation.split_once(" * ") else {
                return Err(anyhow!("Cannot split expression: {}", multiply));
            };

            let operand = operand.parse::<u64>()?;

            Box::new(move |item| item * operand)
        }
        x => return Err(anyhow!("Unhandled expression: {}", x)),
    };

    // test

    if !lines[3].contains("divisible by") {
        return Err(anyhow!("Unexpected Test line: {}", lines[3]));
    }
    let Some((_, divisor)) = lines[3].split_once("divisible by ") else {
        return Err(anyhow!("Cannot split Test line: {}", lines[3]));
    };
    let divisor = divisor.parse::<u64>()?;
    let test = Box::new(move |item: u64| item % divisor == 0);

    // true_target

    if !lines[4].contains("If true") {
        return Err(anyhow!("Unexpected If true line: {}", lines[4]));
    }
    let Some((_, true_target)) = lines[4].split_once("throw to monkey ") else {
        return Err(anyhow!("Cannot split If true line: {}", lines[4]));
    };
    let true_target = true_target.parse::<usize>()?;

    // false_target

    if !lines[5].contains("If false") {
        return Err(anyhow!("Unexpected If false line: {}", lines[5]));
    }
    let Some((_, false_target)) = lines[5].split_once("throw to monkey ") else {
        return Err(anyhow!("Cannot split If false line: {}", lines[5]));
    };
    let false_target = false_target.parse::<usize>()?;

    Ok(Monkey {
        items,
        operation,
        test,
        divisor,
        true_target,
        false_target,
        inspection_count: 0,
    })
}

/// Performs the turn for one monkey.
fn monkey_turn(monkey_id: usize, monkeys: &mut [Monkey], modulus: Option<u64>) {
    let mut items = monkeys[monkey_id].items.clone();

    // Update the monkey first.
    monkeys[monkey_id].items.clear();
    monkeys[monkey_id].inspection_count += items.len();

    while let Some(item) = items.pop_front() {
        let mut item = (monkeys[monkey_id].operation)(item);

        if let Some(modulus) = modulus {
            // Applying Chinese Remainder Theorem, so that item never gets too big to handle, but
            // will still produce the same remainder when checking for divisibility by each monkey.
            // This requires every divisor from all monkeys to be pairwise coprime.
            item %= modulus;
        } else {
            item /= 3;
        }

        let target_id = if (monkeys[monkey_id].test)(item) {
            monkeys[monkey_id].true_target
        } else {
            monkeys[monkey_id].false_target
        };

        // Target monkeys are updated here.
        monkeys[target_id].items.push_back(item);
    }
}

/// Performs one round of items throwing.
fn round(monkeys: &mut [Monkey], modulus: Option<u64>) {
    (0..monkeys.len()).for_each(|monkey_id| monkey_turn(monkey_id, monkeys, modulus));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 10605);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 2713310158);

        Ok(())
    }
}
