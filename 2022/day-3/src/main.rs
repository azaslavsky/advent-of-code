#![feature(iter_array_chunks)]

use anyhow::{bail, Error, Result};
use common::{open_input_file_with_variant, Variant};
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

// Score a letter by priority.
fn char_to_priority(ch: char) -> Result<u32> {
    if ch.is_lowercase() {
        // "a" is the decimal value 97 in ASCII; to make it or any other
        // lower case letters equal 1..=26, we need to subtract 96.
        Ok(ch as u32 - 96)
    } else if ch.is_uppercase() {
        // "A" is the decimal value 65 in ASCII; to make it or any other
        // upper case letters equal 27..=52, we need to subtract 38.
        Ok(ch as u32 - 38)
    } else {
        bail!("invalid item in rucksack")
    }
}

// Find the item common to both the first half and second half of the input list.
fn get_overlapping_item_priority(line: &str) -> Result<u32> {
    let mut seen = HashSet::<u32>::new();
    let count = line.len() / 2;
    if line.len() % 2 != 0 {
        bail!("each rucksack must have an even number of items")
    }

    for (index, ch) in line.chars().enumerate() {
        if index < count {
            seen.insert(char_to_priority(ch)?);
        } else {
            if let Some(priority) = seen.get(&char_to_priority(ch)?) {
                return Ok(*priority);
            }
        }
    }
    bail!("no duplicate item found")
}

// Find the item that occurs in each of the three lines in the |triplet| provided.
fn get_common_item_priority(triplet: [Result<String, std::io::Error>; 3]) -> Result<u32> {
    let mut counts = HashMap::<char, u32>::new();
    for line in triplet {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    bail!("encountered empty line")
                }

                // Use the "seen" set to ensure that we don't count an item multiple times per line.
                let mut seen = HashSet::<char>::new();
                line.chars().for_each(|ch| {
                    if !seen.contains(&ch) {
                        seen.insert(ch);
                        counts
                            .entry(ch)
                            .and_modify(|count| {
                                *count += 1;
                            })
                            .or_insert(1);
                    }
                })
            }
            Err(err) => bail!(err),
        }
    }
    match counts.into_iter().find(|(_, count)| *count == 3) {
        Some((ch, _)) => char_to_priority(ch),
        None => bail!("no common item found"),
    }
}

fn main() -> Result<()> {
    let (input, variant) = open_input_file_with_variant()?;
    let mut lines = io::BufReader::new(input).lines();
    let sum = match variant {
        Variant::A => lines.try_fold(0u32, |acc, line| match line {
            Ok(line) => {
                if !line.is_empty() {
                    return Ok(acc + get_overlapping_item_priority(line.as_str())?);
                }
                bail!("encountered empty line")
            }
            Err(err) => bail!(err),
        })?,
        Variant::B => lines.array_chunks::<3>().try_fold(0u32, |acc, triplet| {
            return Ok::<u32, Error>(acc + get_common_item_priority(triplet)?);
        })?,
    };

    println!("The cumulative priority value is: {}", sum);
    Ok(())
}
