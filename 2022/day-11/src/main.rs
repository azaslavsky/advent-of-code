use anyhow::{bail, Context, Error, Result};
use common::get_input_file_lines_with_variant;
use std::{
    cell::{RefCell, RefMut},
    collections::VecDeque,
    fmt::Debug,
};

// What we divide an `Item`'s `worry` score by whenever a monkey gets bored with it.
const BOREDOM_DIVISOR: usize = 3;

struct Item {
    worry: usize,
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.worry)
    }
}

#[derive(Debug)]
struct Throw {
    to: usize,
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Old,
    Num(usize),
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Op {
    operands: [Operand; 2],
    operator: Operator,
}

#[derive(Debug)]
struct Monkey {
    /// The `Item`s currently held by this `Monkey`.
    items: VecDeque<Item>,

    /// The pre-test operation to perform on the `Item`'s `worry` score.
    op: Op,

    /// How much we divide an `Item` by when testing it.
    test_using: usize,

    /// Which `Monkey` we throw the `Item` to if the test passes.
    if_true: Throw,

    /// Which `Monkey` we throw the `Item` to if the test fails.
    if_false: Throw,

    /// Tracks whether or not the `Monkey` in question has been fully parsed.
    parsed: bool,

    /// Tracks the number of inspections the |Monkey| has done.
    inspections: usize,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: VecDeque::new(),
            op: Op {
                operands: [Operand::Old; 2],
                operator: Operator::Add,
            },
            test_using: 0,
            if_true: Throw { to: 0 },
            if_false: Throw { to: 0 },
            parsed: false,
            inspections: 0,
        }
    }
}

fn parse_items(text: Option<&str>) -> Result<VecDeque<Item>> {
    let mut items = VecDeque::<Item>::new();
    match text {
        None => bail!("parsing: missing item list"),
        Some(text) => text.split(',').try_for_each(|s| {
            Ok::<(), Error>(items.push_back(Item {
                worry: s.trim().parse::<usize>()?,
            }))
        }),
    }?;
    Ok(items)
}

fn parse_operand(text: Option<&str>) -> Result<Operand> {
    match text {
        None => bail!("parsing: missing operand"),
        Some(text) => match text {
            "old" => Ok(Operand::Old),
            _ => Ok(Operand::Num(
                text.parse::<usize>().context("invalid operand")?,
            )),
        },
    }
}

fn parse_operator(text: Option<&str>) -> Result<Operator> {
    match text {
        None => bail!("parsing: missing operator"),
        Some(text) => match text {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => bail!("invalid operator"),
        },
    }
}

fn parse_op(text: Option<&str>) -> Result<Op> {
    match text {
        None => bail!("parsing: missing op"),
        Some(text) => {
            let mut parts = text.trim().split_whitespace().rev().take(3);
            let first = parse_operand(parts.next())?;
            let operator = parse_operator(parts.next())?;
            let second = parse_operand(parts.next())?;
            Ok(Op {
                operands: [first, second],
                operator: operator,
            })
        }
    }
}

fn parse_test(text: Option<&str>) -> Result<usize> {
    match text {
        None => bail!("parsing: missing test"),
        Some(text) => match text.trim().split_whitespace().rev().next() {
            None => bail!("parsing: empty test"),
            Some(divisor) => divisor.parse::<usize>().context("invalid test divisor"),
        },
    }
}

fn parse_throw(text: Option<&str>) -> Result<Throw> {
    match text {
        None => bail!("parsing: missing throw receiver"),
        Some(text) => match text.trim().split_whitespace().rev().next() {
            None => bail!("parsing: empty throw receiver"),
            Some(to) => Ok(Throw {
                to: to.parse::<usize>().context("invalid throw receiver")?,
            }),
        },
    }
}

fn parse(lines: Vec<String>) -> Result<Vec<RefCell<Monkey>>> {
    let state = lines
        .iter()
        .try_fold(Vec::<Monkey>::new(), |mut state, line| {
            if line.is_empty() {
                return Ok(state);
            }

            let mut parts = line.as_str().trim().split(':');
            let kind = parts.next();
            match kind {
                None => bail!("parsing: no colon on instruction line"),
                Some(kind) => {
                    if kind.len() < 4 {
                        bail!("parsing: invalid instruction kind");
                    }
                    match &kind[0..4] {
                        "Monk" => {
                            kind.split_whitespace().next().context(
                                "parsing: no number after `Monkey` on `Monkey ...` line",
                            )?;
                            if let Some(last) = state.last() {
                                if !last.parsed {
                                    bail!("parsing: previous `Monkey` input not done parsing");
                                }
                            }
                            state.push(Monkey::new());
                        }
                        _ => {
                            let last = state
                                .last_mut()
                                .context("parsing: first instruction not `Monkey ...`")?;
                            match kind {
                                "Starting items" => last.items = parse_items(parts.next())?,
                                "Operation" => last.op = parse_op(parts.next())?,
                                "Test" => last.test_using = parse_test(parts.next())?,
                                "If true" => last.if_true = parse_throw(parts.next())?,
                                "If false" => {
                                    last.if_false = parse_throw(parts.next())?;
                                    last.parsed = true
                                }
                                _ => bail!("parsing: invalid line content prior to colon"),
                            }
                        }
                    }
                }
            };
            Ok(state)
        })?;
    Ok(state
        .into_iter()
        .map(|monkey| RefCell::new(monkey))
        .collect::<Vec<_>>())
}

fn throw_items(monkey: &mut RefMut<Monkey>, state: &mut Vec<RefCell<Monkey>>) -> Result<()> {
    monkey.inspections += monkey.items.iter().try_fold(0, |acc, item| {
        let op = &monkey.op;
        let a = match op.operands[0] {
            Operand::Old => item.worry,
            Operand::Num(num) => num,
        };
        let b = match op.operands[1] {
            Operand::Old => item.worry,
            Operand::Num(num) => num,
        };
        let new_worry = match op.operator {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
        } / BOREDOM_DIVISOR;
        let throw_to = match new_worry % monkey.test_using == 0 {
            true => monkey.if_true.to,
            false => monkey.if_false.to,
        };
        let new_owner = state.get_mut(throw_to).context(format!(
            "attempting to throw to non-existing monkey: {}",
            throw_to
        ))?;
        new_owner
            .try_borrow_mut()?
            .items
            .push_back(Item { worry: new_worry });
        Ok::<usize, Error>(acc + 1)
    })?;
    monkey.items.clear();
    Ok(())
}

fn play_round(state: &mut Vec<RefCell<Monkey>>) -> Result<()> {
    // :(
    unsafe {
        let s = &mut *(state as *mut Vec<RefCell<Monkey>>);
        state.iter_mut().try_for_each(|monkey| {
            throw_items(&mut monkey.try_borrow_mut()?, s)?;
            Ok::<(), Error>(())
        })?;
    }
    Ok(())
}

#[allow(dead_code)]
fn print_state(state: &mut Vec<RefCell<Monkey>>) {
    println!("Parsed state: {:#?}", state);
}

fn print_round(state: &mut Vec<RefCell<Monkey>>, num: usize) -> Result<()> {
    println!(
        "\n After round {}, the monkeys are holding items with these worry levels:",
        num
    );
    state.iter().enumerate().try_for_each(|(index, monkey)| {
        println!("Monkey {}: {:?}", index, monkey.try_borrow()?.items);
        Ok::<(), Error>(())
    })?;
    Ok(())
}

/// The number of rounds we track the monkeys for before doing calculations.
const ROUNDS: usize = 20;

fn main() -> Result<()> {
    let (lines, _variant) = get_input_file_lines_with_variant()?;
    let mut state = parse(lines)?;
    // print_state(&mut state);

    for round in 0..ROUNDS {
        play_round(&mut state)?;
        print_round(&mut state, round + 1)?;
    }

    let mut inspections = state
        .iter()
        .enumerate()
        .map(|(_, monkey)| monkey.borrow().inspections)
        .collect::<Vec<_>>();
    inspections.sort();
    
    println!(
        "\nThe level of monkey business is: {}",
        inspections
            .iter()
            .rev()
            .take(2)
            .fold(1, |acc, inspections| acc * inspections)
    );
    Ok(())
}
