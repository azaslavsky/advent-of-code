use anyhow::{Error, Result};
use common::{open_input_file_with_variant, Variant};
use std::io::{self, BufRead};

// Assumes two ranges, each with two digits.
fn superset(mut ranges: Vec<Vec<u32>>) -> bool {
    ranges[0].sort();
    ranges[1].sort();
    ranges.sort();
    let lower = &ranges[0];
    let upper = &ranges[1];
    if upper[0] == lower[0] || upper[1] <= lower[1] {
        return true;
    }
    return false;
}

// Assumes two ranges, each with two digits.
fn intersect(mut ranges: Vec<Vec<u32>>) -> bool {
    ranges[0].sort();
    ranges[1].sort();
    ranges.sort();
    let lower = &ranges[0];
    let upper = &ranges[1];
    if lower[1] >= upper[0] {
        return true;
    }
    return false;
}

fn main() -> Result<()> {
    let (input, variant) = open_input_file_with_variant()?;
    let sum = io::BufReader::new(input)
        .lines()
        .try_fold(0u32, |acc, line| {
            let line = line?;
            if line.is_empty() {
                return Ok::<u32, Error>(acc)
            }

            let ranges = line
                .split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|num| num.parse::<u32>().unwrap())
                        .take(2)
                        .collect::<Vec<_>>()
                })
                .take(2)
                .collect::<Vec<_>>();
            Ok(acc
                + match variant {
                    Variant::A => superset(ranges),
                    Variant::B => intersect(ranges),
                } as u32)
        })?;

    println!("The number of fully-contained pairs is: {}", sum);
    Ok(())
}
