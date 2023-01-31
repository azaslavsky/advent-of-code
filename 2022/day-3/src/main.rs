use anyhow::{bail, Result};
use common::open_input_file_with_variant;
use std::collections::HashSet;
use std::io::{self, BufRead};

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

fn main() -> Result<()> {
    let (input, _variant) = open_input_file_with_variant()?;
    let sum = io::BufReader::new(input)
        .lines()
        .try_fold(0u32, |acc, line| match line {
            Ok(line) => {
                if !line.is_empty() {
                    let mut seen = HashSet::<u32>::new();
                    let count = line.len()/2;
                    if line.len() % 2 != 0 {
                        bail!("each rucksack must have an even number of items")
                    }

                    for (index, ch) in line.chars().enumerate() {
                        if index < count {
                            seen.insert(char_to_priority(ch)?);
                        } else {
                            if let Some(priority) = seen.get(&char_to_priority(ch)?) {
                                return Ok(acc + priority);
                            }
                        }
                    }
                }
                bail!("no duplicate item found")
            }
            Err(err) => bail!(err),
        })?;

    println!("The cumulative priority value is: {}", sum);
    Ok(())
}
