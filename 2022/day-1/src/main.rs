use anyhow::{bail, Result};
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::Context;
use std::collections::BTreeMap;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Must specify exactly one argument, a file to load as input");
    }
    let file_path = &args[1];

    let input = File::open(file_path).context("could not find input file")?;
    let mut loads = BTreeMap::<u32, u8>::new();
    io::BufReader::new(input)
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

            loads.entry(acc).and_modify(|count| *count += 1).or_insert(1);
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
