use anyhow::{bail, Context, Result};
use common::get_input_file_lines_with_variant;
use std::{
    cmp::{min, Ordering, PartialOrd},
    str::Chars,
};

#[derive(Debug, PartialEq)]
enum Item {
    Num(Integer),
    List(List),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Item::Num(self_num) => match other {
                Item::Num(other_num) => self_num.partial_cmp(other_num),
                Item::List(other_list) => List {
                    items: vec![Item::Num(*self_num)],
                }
                .partial_cmp(other_list),
            },
            Item::List(self_list) => match other {
                Item::Num(other_num) => self_list.partial_cmp(&List {
                    items: vec![Item::Num(*other_num)],
                }),
                Item::List(other_list) => self_list.partial_cmp(other_list),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Integer {
    value: u32,
}

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.value == other.value {
            Ordering::Equal
        } else {
            if self.value < other.value {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
    }
}

#[derive(Debug, Default, PartialEq)]
struct List {
    items: Vec<Item>,
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let min_len = min(self.items.len(), other.items.len());
        for i in 0..min_len {
            if self.items[i] < other.items[i] {
                return Some(Ordering::Less);
            } else if self.items[i] > other.items[i] {
                return Some(Ordering::Greater);
            }
        }
        Some(if &self.items.len() < &other.items.len() {
            Ordering::Less
        } else if &self.items.len() > &other.items.len() {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }
}

fn parse_list(chars: &mut Chars) -> Result<List> {
    let mut out = List::default();
    let mut last_was_digit = false;
    let mut c = chars.next();
    while c != None {
        let ch = c.unwrap();
        if ch.is_digit(10) {
            let digit = ch
                .to_digit(10)
                .context("parsing: unable to convert validated digit")?;
            if !last_was_digit {
                out.items.push(Item::Num(Integer { value: digit }))
            } else {
                match out
                    .items
                    .last_mut()
                    .context("parsing: cannot modify non-existent number")?
                {
                    Item::List(_) => bail!("parsing: expected to modify digit"),
                    Item::Num(num) => {
                        num.value *= 10;
                        num.value += digit;
                    }
                }
            }
            last_was_digit = true;
        } else {
            last_was_digit = false;
            match ch {
                '[' => {
                    out.items.push(Item::List(parse_list(chars)?));
                }
                ']' => return Ok(out),
                ',' => {}
                _ => bail!("parsing: invalid character"),
            }
        }
        c = chars.next();
    }
    bail!("parsing: all open brackets should have a matching closing pair")
}

fn compare_lists(left: &Option<List>, right: &Option<List>) -> Result<bool> {
    match &left {
        None => bail!("parsing: zero lists provided for comparison"),
        Some(l) => match &right {
            None => bail!("parsing: only one list provided for comparison"),
            Some(r) => {
                if l < r {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        },
    }
}

fn compare_all(lines: Vec<String>) -> Result<Vec<Option<usize>>> {
    let mut left = None;
    let mut right = None;
    let mut results = lines
        .iter()
        .enumerate()
        .try_fold(Vec::new(), |mut acc, (index, line)| {
            if line.is_empty() {
                acc.push(match compare_lists(&left, &right)? {
                    false => None,
                    true => Some((index + 1) / 3),
                });
                left = None;
                right = None;
            } else {
                let mut chars = line.chars();
                if chars
                    .next()
                    .context("parsing: list line with no opening character")?
                    != '['
                {
                    bail!("parsing: list line with no opening bracket")
                }

                let entry = parse_list(&mut chars)?;
                if left.is_none() {
                    left = Some(entry);
                } else if right.is_none() {
                    right = Some(entry);
                } else {
                    bail!("parsing: three consecutive lists not allowed");
                }
            }
            Ok(acc)
        })?;

    if left.is_some() && right.is_some() {
        results.push(match compare_lists(&left, &right)? {
            false => None,
            true => Some((lines.len() + 1) / 3),
        });
    }
    Ok(results)
}

fn main() -> Result<()> {
    let (lines, _variant) = get_input_file_lines_with_variant()?;
    let sum = compare_all(lines)?
        .iter()
        .fold(0, |acc, maybe_val| acc + maybe_val.unwrap_or(0));

    println!(
        "The sum of the indices of the correctly ordered pairs is: {}",
        sum
    );
    Ok(())
}
