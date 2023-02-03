use anyhow::{bail, Context, Error, Result};
use common::{get_input_file_lines_with_variant, Variant};
use std::{
    cell::{RefCell, RefMut},
    collections::VecDeque,
    fmt::Debug,
};

// What we divide an `Item`'s `worry` score by whenever a monkey gets bored with it.
const BOREDOM_DIVISOR_A: usize = 3;
const BOREDOM_DIVISOR_B: usize = 1;

trait Worry
where
    Self: Sized,
{
    fn parse(from: &str) -> Result<Self>;
    fn from_usize(from: usize) -> Self;
    fn try_divide_by(self, divisor: usize) -> Result<Self>;
    fn plus(&mut self, rhs: &Self);
    fn times(&mut self, rhs: &Self);
}

impl Worry for usize {
    fn parse(from: &str) -> Result<usize> {
        Ok(from.parse()?)
    }

    fn from_usize(from: usize) -> usize {
        from
    }

    fn try_divide_by(self, divisor: usize) -> Result<usize> {
        Ok(self)
    }

    fn plus(&mut self, rhs: &Self) {
        *self += rhs;
    }

    fn times(&mut self, rhs: &Self) {
        *self *= rhs;
    }
}

impl Worry for Vec<usize> {
    fn parse(from: &str) -> Result<Vec<usize>> {
        let mut out = Vec::<usize>::new();
        for ch in from.chars() {
            out.push(ch.to_digit(10).context("parsing: invalid character")? as usize);
        }
        Ok(out.into_iter().rev().collect())
    }

    fn from_usize(from: usize) -> Vec<usize> {
        vec![from]
    }

    fn try_divide_by(self, divisor: usize) -> Result<Vec<usize>> {
        if divisor == 1 {
            return Ok(self);
        }
        bail!("there should be no need for division in variant b");
    }

    fn plus(&mut self, rhs: &Self) {}

    fn times(&mut self, rhs: &Self) {}
}

struct Item<W> {
    worry: W,
}

impl<W> Debug for Item<W>
where
    W: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.worry)
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
struct Monkey<W>
where
    W: Debug,
{
    /// The `Item`s currently held by this `Monkey`.
    items: VecDeque<Item<W>>,

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

impl<W> Monkey<W>
where
    W: Debug,
{
    fn new() -> Monkey<W> {
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

fn parse_items<W>(text: Option<&str>) -> Result<VecDeque<Item<W>>>
where
    W: Debug + Worry,
{
    let mut items = VecDeque::<Item<W>>::new();
    match text {
        None => bail!("parsing: missing item list"),
        Some(text) => text.split(',').try_for_each(|s| {
            Ok::<(), Error>(items.push_back(Item {
                worry: W::parse(text)?,
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

fn parse<W>(lines: Vec<String>) -> Result<Vec<RefCell<Monkey<W>>>>
where
    W: Debug + Worry,
{
    let state = lines
        .iter()
        .try_fold(Vec::<Monkey<W>>::new(), |mut state, line| {
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

fn throw_items<W>(
    variant: &Variant,
    monkey: &mut RefMut<Monkey<W>>,
    state: &mut Vec<RefCell<Monkey<W>>>,
) -> Result<()>
where
    W: Debug + Worry,
{
    monkey.inspections += monkey.items.iter().try_fold(0, |acc, item| {
        let op = &monkey.op;
        let a = match op.operands[0] {
            Operand::Old => item.worry,
            Operand::Num(num) => W::from_usize(num),
        };
        let b = match op.operands[1] {
            Operand::Old => item.worry,
            Operand::Num(num) => W::from_usize(num),
        };
        let mut new_worry = match op.operator {
            Operator::Add => a.plus(&b),
            Operator::Multiply => a.times(&b),
        };
        new_worry = new_worry.try_divide_by(match variant {
            Variant::A => BOREDOM_DIVISOR_A,
            Variant::B => BOREDOM_DIVISOR_B,
        })?;
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

fn play_round<W>(variant: &Variant, state: &mut Vec<RefCell<Monkey<W>>>) -> Result<()>
where
    W: Debug + Worry,
{
    // :(
    unsafe {
        let s = &mut *(state as *mut Vec<RefCell<Monkey<W>>>);
        state.iter_mut().try_for_each(|monkey| {
            throw_items(variant, &mut monkey.try_borrow_mut()?, s)?;
            Ok::<(), Error>(())
        })?;
    }
    Ok(())
}

#[allow(dead_code)]
fn print_state<W>(state: &mut Vec<RefCell<Monkey<W>>>)
where
    W: Debug,
{
    println!("Parsed state: {:#?}", state);
}

fn print_round<W>(state: &mut Vec<RefCell<Monkey<W>>>, num: usize) -> Result<()>
where
    W: Debug,
{
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
const ROUNDS_A: usize = 20;
const ROUNDS_B: usize = 1000;

fn main() -> Result<()> {
    let (lines, variant) = get_input_file_lines_with_variant()?;

    let mut inspections = match &variant {
        Variant::A => {
            let mut state = parse::<usize>(lines)?;
            // print_state(&mut state);
            for round in 0..ROUNDS_A {
                play_round(&variant, &mut state)?;
                print_round(&mut state, round + 1)?;
            }
            state
                .iter()
                .enumerate()
                .map(|(_, monkey)| monkey.borrow().inspections)
                .collect::<Vec<_>>()
        }
        // Variant::B => {}
        _ => todo!(),
    };

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
