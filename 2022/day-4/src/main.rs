use anyhow::{bail, Result};
use common::open_input_file_with_variant;
use std::io::{self, BufRead};

// Assumes two ranges, each with two digits.
fn superset(mut ranges: Vec<Vec<u32>>) -> bool {
    ranges.sort();
    ranges[0].sort();
    ranges[1].sort();
    let lower = &ranges[0];
    let upper = &ranges[1];
    if lower[0] == upper[0] || upper[1] <= lower[1] {
        return true;
    }
    return false;
}

fn main() -> Result<()> {
    let (input, _variant) = open_input_file_with_variant()?;
    let sum = io::BufReader::new(input)
        .lines()
        .try_fold(0u32, |acc, line| match line {
            Ok(line) => {
                if !line.is_empty() {
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
                    if superset(ranges) {
                        return Ok(acc + 1);
                    }
                }
                Ok(acc)
            }
            Err(err) => bail!(err),
        })?;

    println!("The number of fully-contained pairs is: {}", sum);
    Ok(())
}
