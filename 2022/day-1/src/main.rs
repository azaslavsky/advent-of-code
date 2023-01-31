use anyhow::{Context, Result};
use common::open_input_file;
use std::collections::BTreeMap;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut loads = BTreeMap::<u32, u8>::new();
    io::BufReader::new(open_input_file()?)
        .lines()
        .try_fold(0u32, |acc, line| {
            match line {
                Ok(line) => {
                    if !line.is_empty() {
                        let val = line.parse::<u32>().context("invalid input line")?;
                        return Ok::<u32, anyhow::Error>(acc + val);
                    }
                }
                _ => {}
            }

            loads
                .entry(acc)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            Ok(0)
        })?;

    let mut sum = 0u32;
    let mut left = 3u8;
    for (calories, count) in loads.iter().rev() {
        if count >= &left {
            sum += left as u32 * calories;
            break;
        }

        left -= count;
        sum += *count as u32 * calories;
    }
    println!(
        "The maximum number of calories carried by the three most laden elves is: {}",
        sum
    );
    Ok(())
}
